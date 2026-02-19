// <FILE>src/rendering/cls_ratatui_buffer_adapter.rs</FILE> - <DESC>Adapter wrapping ratatui::Buffer to implement Grid trait</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor integration - enable render_pipeline to work with ratatui buffers</WCTX>
// <CLOG>Initial creation - implement Grid trait for ratatui::Buffer compatibility</CLOG>

//! Ratatui Buffer adapter for the tui-vfx Grid trait.
//!
//! This adapter bridges the framework-agnostic compositor (`tui-vfx-compositor`)
//! with the ratatui rendering backend. It allows `render_pipeline` to operate
//! on a ratatui `Buffer` through the `Grid` trait abstraction.
//!
//! ## Example
//!
//! ```ignore
//! use ratatui::buffer::Buffer;
//! use ratatui::layout::Rect;
//! use tui_vfx_compositor::pipeline::render_pipeline;
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buffer = Buffer::empty(area);
//! let mut adapter = RatatuiBufferAdapter::new(&mut buffer);
//!
//! // Now render_pipeline can use the adapter as a Grid
//! render_pipeline(source, &mut adapter, width, height, 0, 0, options, None);
//! ```

use ratatui::buffer::Buffer;
use tui_vfx_types::{Cell, Grid};

use crate::compat::{apply_vfx_cell_to_ratatui, ratatui_cell_to_vfx};

/// Adapter that wraps a ratatui `Buffer` to implement the `Grid` trait.
///
/// This enables the framework-agnostic compositor pipeline to render
/// effects directly into a ratatui buffer.
///
/// The adapter handles coordinate translation between the Grid's (x, y)
/// coordinates and the Buffer's internal representation.
pub struct RatatuiBufferAdapter<'a> {
    buffer: &'a mut Buffer,
    /// Base offset for x coordinates (relative to buffer area).
    offset_x: u16,
    /// Base offset for y coordinates (relative to buffer area).
    offset_y: u16,
}

impl<'a> RatatuiBufferAdapter<'a> {
    /// Create a new adapter wrapping a ratatui Buffer.
    ///
    /// The adapter will use the full buffer area with no offset.
    pub fn new(buffer: &'a mut Buffer) -> Self {
        Self {
            buffer,
            offset_x: 0,
            offset_y: 0,
        }
    }

    /// Create an adapter with a coordinate offset.
    ///
    /// This is useful when rendering into a sub-region of the buffer.
    /// The offset is added to all coordinate operations.
    pub fn with_offset(buffer: &'a mut Buffer, offset_x: u16, offset_y: u16) -> Self {
        Self {
            buffer,
            offset_x,
            offset_y,
        }
    }

    /// Get the underlying buffer's area.
    pub fn area(&self) -> ratatui::layout::Rect {
        self.buffer.area
    }

    /// Translate Grid coordinates to buffer coordinates.
    #[inline]
    fn translate(&self, x: usize, y: usize) -> (u16, u16) {
        (
            self.offset_x.saturating_add(x as u16),
            self.offset_y.saturating_add(y as u16),
        )
    }

    /// Check if translated coordinates are within buffer bounds.
    #[inline]
    fn is_in_buffer(&self, buf_x: u16, buf_y: u16) -> bool {
        let area = self.buffer.area;
        buf_x >= area.x
            && buf_x < area.x.saturating_add(area.width)
            && buf_y >= area.y
            && buf_y < area.y.saturating_add(area.height)
    }
}

impl Grid for RatatuiBufferAdapter<'_> {
    fn width(&self) -> usize {
        self.buffer.area.width as usize
    }

    fn height(&self) -> usize {
        self.buffer.area.height as usize
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        // We cannot return a reference to a VfxCell directly because
        // the buffer stores ratatui::Cell. We need to use get_mut pattern
        // or convert on-demand. For the Grid trait, we return None here
        // and rely on set() for mutations. The compositor uses get() mainly
        // for reading source cells, which should come from a separate Grid.
        //
        // For source reading, create a snapshot grid. For the destination
        // buffer, only set() is needed.
        let (buf_x, buf_y) = self.translate(x, y);
        if self.is_in_buffer(buf_x, buf_y) {
            // Note: This is a limitation - we can't return &Cell because
            // the underlying storage is ratatui::Cell. The caller should
            // use a snapshot grid for reading or implement a caching layer.
            None
        } else {
            None
        }
    }

    fn get_mut(&mut self, _x: usize, _y: usize) -> Option<&mut Cell> {
        // Same limitation as get() - the underlying storage type differs.
        // Use set() for modifications.
        None
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let (buf_x, buf_y) = self.translate(x, y);
        if self.is_in_buffer(buf_x, buf_y) {
            // Get mutable reference to the ratatui cell and apply the vfx cell
            if let Some(ratatui_cell) = self.buffer.cell_mut((buf_x, buf_y)) {
                apply_vfx_cell_to_ratatui(cell, ratatui_cell);
            }
        }
    }
}

/// A snapshot of a ratatui Buffer that can be used as a read-only Grid source.
///
/// This creates a copy of the buffer's cells converted to VfxCell format,
/// allowing the compositor to read from it while writing to a separate destination.
pub struct RatatuiBufferSnapshot {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl RatatuiBufferSnapshot {
    /// Create a snapshot from a ratatui Buffer.
    ///
    /// This copies and converts all cells, so it has O(width * height) cost.
    pub fn from_buffer(buffer: &Buffer) -> Self {
        let width = buffer.area.width as usize;
        let height = buffer.area.height as usize;
        let mut cells = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let buf_x = buffer.area.x + x as u16;
                let buf_y = buffer.area.y + y as u16;
                if let Some(ratatui_cell) = buffer.cell((buf_x, buf_y)) {
                    cells.push(ratatui_cell_to_vfx(ratatui_cell));
                } else {
                    cells.push(Cell::default());
                }
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    /// Create a snapshot from a specific region of a ratatui Buffer.
    pub fn from_region(buffer: &Buffer, x: u16, y: u16, width: u16, height: u16) -> Self {
        let width_usize = width as usize;
        let height_usize = height as usize;
        let mut cells = Vec::with_capacity(width_usize * height_usize);

        for dy in 0..height {
            for dx in 0..width {
                let buf_x = x + dx;
                let buf_y = y + dy;
                if let Some(ratatui_cell) = buffer.cell((buf_x, buf_y)) {
                    cells.push(ratatui_cell_to_vfx(ratatui_cell));
                } else {
                    cells.push(Cell::default());
                }
            }
        }

        Self {
            cells,
            width: width_usize,
            height: height_usize,
        }
    }

    /// Get the linear index for (x, y) coordinates.
    #[inline]
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

impl Grid for RatatuiBufferSnapshot {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.index(x, y).map(|idx| &self.cells[idx])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.index(x, y).map(|idx| &mut self.cells[idx])
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(idx) = self.index(x, y) {
            self.cells[idx] = cell;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;
    use tui_vfx_types::Color;

    #[test]
    fn test_adapter_dimensions() {
        let area = Rect::new(0, 0, 80, 24);
        let mut buffer = Buffer::empty(area);
        let adapter = RatatuiBufferAdapter::new(&mut buffer);

        assert_eq!(adapter.width(), 80);
        assert_eq!(adapter.height(), 24);
    }

    #[test]
    fn test_adapter_set_cell() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buffer = Buffer::empty(area);
        {
            let mut adapter = RatatuiBufferAdapter::new(&mut buffer);

            let cell = Cell::styled(
                'X',
                Color::RED,
                Color::BLUE,
                tui_vfx_types::Modifiers::bold(),
            );
            adapter.set(5, 2, cell);
        }

        // Verify the cell was written
        let ratatui_cell = buffer.cell((5, 2)).unwrap();
        assert_eq!(ratatui_cell.symbol(), "X");
    }

    #[test]
    fn test_adapter_with_offset() {
        let area = Rect::new(0, 0, 20, 10);
        let mut buffer = Buffer::empty(area);
        {
            let mut adapter = RatatuiBufferAdapter::with_offset(&mut buffer, 5, 3);

            let cell = Cell::new('O');
            adapter.set(0, 0, cell);
        }

        // Cell at adapter (0,0) should be at buffer (5,3)
        let ratatui_cell = buffer.cell((5, 3)).unwrap();
        assert_eq!(ratatui_cell.symbol(), "O");
    }

    #[test]
    fn test_adapter_out_of_bounds() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buffer = Buffer::empty(area);
        let mut adapter = RatatuiBufferAdapter::new(&mut buffer);

        // This should not panic, just be a no-op
        let cell = Cell::new('!');
        adapter.set(100, 100, cell);
    }

    #[test]
    fn test_snapshot_from_buffer() {
        let area = Rect::new(0, 0, 5, 3);
        let mut buffer = Buffer::empty(area);

        // Set some content
        buffer.cell_mut((2, 1)).unwrap().set_char('A');

        let snapshot = RatatuiBufferSnapshot::from_buffer(&buffer);

        assert_eq!(snapshot.width(), 5);
        assert_eq!(snapshot.height(), 3);

        // Check the cell we set
        let cell = snapshot.get(2, 1).unwrap();
        assert_eq!(cell.ch, 'A');

        // Check default cells
        let default_cell = snapshot.get(0, 0).unwrap();
        assert_eq!(default_cell.ch, ' ');
    }

    #[test]
    fn test_snapshot_grid_trait() {
        let area = Rect::new(0, 0, 4, 4);
        let buffer = Buffer::empty(area);
        let mut snapshot = RatatuiBufferSnapshot::from_buffer(&buffer);

        // Test set and get
        let cell = Cell::new('Z');
        snapshot.set(1, 1, cell);

        let retrieved = snapshot.get(1, 1).unwrap();
        assert_eq!(retrieved.ch, 'Z');
    }
}

// <FILE>src/rendering/cls_ratatui_buffer_adapter.rs</FILE> - <DESC>Adapter wrapping ratatui::Buffer to implement Grid trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
