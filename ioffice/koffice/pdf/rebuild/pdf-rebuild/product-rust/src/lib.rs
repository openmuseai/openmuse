//! Minimal safe Rust owner for the existing product C ABI. This proves the
//! language boundary; Flutter still calls the C ABI directly in production.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;

#[repr(C)]
#[derive(Default)]
struct ProductBytes {
    data: *mut u8,
    size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NativeTextChar {
    utf16: u32,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

#[repr(C)]
#[derive(Default)]
struct NativePageText {
    chars: *mut NativeTextChar,
    count: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NativePageLink {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
    kind: i32,
    page_index: i32,
    uri_offset: u32,
    uri_length: u32,
}

#[repr(C)]
#[derive(Default)]
struct NativePageLinks {
    links: *mut NativePageLink,
    count: i32,
    uri_data: *mut u8,
    uri_size: u32,
}

#[link(name = "kspfreetext")]
extern "C" {
    fn ksp_product_edit_abi_version() -> u32;
    fn ksp_product_edit_open_bytes(bytes: *const u8, size: u64, error: *mut i32) -> *mut c_void;
    fn ksp_product_edit_page_count(session: *mut c_void) -> i32;
    fn ksp_product_edit_page_size(
        session: *mut c_void,
        page_index: i32,
        width: *mut f32,
        height: *mut f32,
    ) -> i32;
    fn ksp_product_edit_render_page_at(
        session: *mut c_void,
        page_index: i32,
        width: i32,
        height: i32,
        rgba: *mut u8,
        capacity: i32,
    ) -> i32;
    fn ksp_product_edit_render_page_region(
        session: *mut c_void,
        page_index: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        full_width: i32,
        full_height: i32,
        rgba: *mut u8,
        capacity: i32,
    ) -> i32;
    fn ksp_product_edit_read_page_text(
        session: *mut c_void,
        page_index: i32,
        output: *mut NativePageText,
    ) -> i32;
    fn ksp_product_edit_free_page_text(output: *mut NativePageText);
    fn ksp_product_edit_read_page_links(
        session: *mut c_void,
        page_index: i32,
        output: *mut NativePageLinks,
    ) -> i32;
    fn ksp_product_edit_free_page_links(output: *mut NativePageLinks);
    fn ksp_product_edit_export_bytes(session: *mut c_void, output: *mut ProductBytes) -> i32;
    fn ksp_product_free_bytes(output: *mut ProductBytes);
    fn ksp_product_edit_close(session: *mut c_void);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProductError {
    EmptyInput,
    IncompatibleAbi(u32),
    Native(i32),
    InvalidOutput,
}

#[derive(Clone, Debug)]
pub struct ProductTextCharacter {
    pub code_unit: u16,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductLinkTarget {
    Unsupported,
    Uri(Vec<u8>),
    Page(u32),
}

#[derive(Clone, Debug)]
pub struct ProductPageLink {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
    pub target: ProductLinkTarget,
}

/// Owns the native PDF document and closes it exactly once.
pub struct ProductSession {
    handle: NonNull<c_void>,
    // Native PDFium session and font callbacks are not thread-safe yet.
    _not_thread_safe: PhantomData<Rc<()>>,
}

impl ProductSession {
    pub fn open(bytes: &[u8]) -> Result<Self, ProductError> {
        if bytes.is_empty() {
            return Err(ProductError::EmptyInput);
        }
        // SAFETY: no pointers are retained except the native session returned
        // on success; the native bridge copies its input document bytes.
        unsafe {
            let version = ksp_product_edit_abi_version();
            if version != 6 {
                return Err(ProductError::IncompatibleAbi(version));
            }
            let mut error = 0;
            let handle =
                ksp_product_edit_open_bytes(bytes.as_ptr(), bytes.len() as u64, &mut error);
            NonNull::new(handle)
                .map(|handle| Self {
                    handle,
                    _not_thread_safe: PhantomData,
                })
                .ok_or(if error < 0 {
                    ProductError::Native(error)
                } else {
                    ProductError::InvalidOutput
                })
        }
    }

    pub fn page_count(&self) -> Result<u32, ProductError> {
        // SAFETY: self owns a live native session until Drop.
        let count = unsafe { ksp_product_edit_page_count(self.handle.as_ptr()) };
        if count < 0 {
            Err(ProductError::Native(count))
        } else {
            Ok(count as u32)
        }
    }

    pub fn snapshot(&self) -> Result<Vec<u8>, ProductError> {
        let mut output = ProductBytes::default();
        // SAFETY: the native API initializes output and owns its allocation.
        let status = unsafe { ksp_product_edit_export_bytes(self.handle.as_ptr(), &mut output) };
        let result = if status < 0 {
            Err(ProductError::Native(status))
        } else if output.data.is_null() || output.size == 0 || output.size > isize::MAX as u64 {
            Err(ProductError::InvalidOutput)
        } else {
            // SAFETY: successful native output is valid for its reported size
            // until ksp_product_free_bytes runs below.
            Ok(unsafe { std::slice::from_raw_parts(output.data, output.size as usize).to_vec() })
        };
        // SAFETY: the native allocator must release output, including failures.
        unsafe { ksp_product_free_bytes(&mut output) };
        result
    }

    pub fn page_size(&self, page_index: i32) -> Result<(f32, f32), ProductError> {
        let (mut width, mut height) = (0.0, 0.0);
        // SAFETY: self owns the session and both output pointers are writable.
        let status = unsafe {
            ksp_product_edit_page_size(self.handle.as_ptr(), page_index, &mut width, &mut height)
        };
        if status < 0 {
            Err(ProductError::Native(status))
        } else if width <= 0.0 || height <= 0.0 {
            Err(ProductError::InvalidOutput)
        } else {
            Ok((width, height))
        }
    }

    pub fn read_page_text(
        &self,
        page_index: i32,
    ) -> Result<Vec<ProductTextCharacter>, ProductError> {
        let mut output = NativePageText::default();
        // SAFETY: the native call initializes output and owns its allocation.
        let status = unsafe {
            ksp_product_edit_read_page_text(self.handle.as_ptr(), page_index, &mut output)
        };
        let result = if status < 0 {
            Err(ProductError::Native(status))
        } else if output.count < 0
            || output.count > 1_000_000
            || (output.count > 0 && output.chars.is_null())
        {
            Err(ProductError::InvalidOutput)
        } else {
            let slice = if output.count == 0 {
                &[][..]
            } else {
                // SAFETY: successful native output remains valid until free below.
                unsafe { std::slice::from_raw_parts(output.chars, output.count as usize) }
            };
            Ok(slice
                .iter()
                .map(|ch| ProductTextCharacter {
                    code_unit: ch.utf16 as u16,
                    left: ch.left,
                    bottom: ch.bottom,
                    right: ch.right,
                    top: ch.top,
                })
                .collect())
        };
        // SAFETY: this releases output on both success and failure.
        unsafe { ksp_product_edit_free_page_text(&mut output) };
        result
    }

    pub fn read_page_links(&self, page_index: i32) -> Result<Vec<ProductPageLink>, ProductError> {
        let mut output = NativePageLinks::default();
        // SAFETY: the native call initializes output and owns both allocations.
        let status = unsafe {
            ksp_product_edit_read_page_links(self.handle.as_ptr(), page_index, &mut output)
        };
        let result = if status < 0 {
            Err(ProductError::Native(status))
        } else if output.count < 0
            || output.count > 4096
            || output.uri_size > 1024 * 1024
            || (output.count > 0 && output.links.is_null())
            || (output.uri_size > 0 && output.uri_data.is_null())
        {
            Err(ProductError::InvalidOutput)
        } else {
            let links = if output.count == 0 {
                &[][..]
            } else {
                // SAFETY: native output stays live until the free call below.
                unsafe { std::slice::from_raw_parts(output.links, output.count as usize) }
            };
            let uri_data = if output.uri_size == 0 {
                &[][..]
            } else {
                // SAFETY: native output stays live until the free call below.
                unsafe { std::slice::from_raw_parts(output.uri_data, output.uri_size as usize) }
            };
            links
                .iter()
                .map(|link| {
                    let target = match link.kind {
                        1 => {
                            let start = link.uri_offset as usize;
                            let end = start
                                .checked_add(link.uri_length as usize)
                                .ok_or(ProductError::InvalidOutput)?;
                            ProductLinkTarget::Uri(
                                uri_data
                                    .get(start..end)
                                    .ok_or(ProductError::InvalidOutput)?
                                    .to_vec(),
                            )
                        }
                        2 if link.page_index >= 0 => {
                            ProductLinkTarget::Page(link.page_index as u32)
                        }
                        _ => ProductLinkTarget::Unsupported,
                    };
                    Ok(ProductPageLink {
                        left: link.left,
                        bottom: link.bottom,
                        right: link.right,
                        top: link.top,
                        target,
                    })
                })
                .collect()
        };
        // SAFETY: native allocator releases both arrays, including failures.
        unsafe { ksp_product_edit_free_page_links(&mut output) };
        result
    }

    pub fn render_page(
        &self,
        page_index: i32,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, ProductError> {
        let pixels = width
            .checked_mul(height)
            .ok_or(ProductError::InvalidOutput)?;
        if pixels == 0 || pixels > 16 * 1024 * 1024 {
            return Err(ProductError::InvalidOutput);
        }
        let capacity = pixels * 4;
        let mut rgba = vec![0; capacity as usize];
        // SAFETY: the buffer remains writable for the entire synchronous call.
        let written = unsafe {
            ksp_product_edit_render_page_at(
                self.handle.as_ptr(),
                page_index,
                width as i32,
                height as i32,
                rgba.as_mut_ptr(),
                capacity as i32,
            )
        };
        if written < 0 {
            Err(ProductError::Native(written))
        } else if written != capacity as i32 {
            Err(ProductError::InvalidOutput)
        } else {
            Ok(rgba)
        }
    }

    pub fn render_region(
        &self,
        page_index: i32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        full_width: u32,
        full_height: u32,
    ) -> Result<Vec<u8>, ProductError> {
        let pixels = width
            .checked_mul(height)
            .ok_or(ProductError::InvalidOutput)?;
        if pixels == 0
            || pixels > 16 * 1024 * 1024
            || full_width > i32::MAX as u32
            || full_height > i32::MAX as u32
            || x > full_width
            || y > full_height
            || width > full_width - x
            || height > full_height - y
        {
            return Err(ProductError::InvalidOutput);
        }
        let capacity = pixels * 4;
        let mut rgba = vec![0; capacity as usize];
        // SAFETY: the buffer remains writable for the entire synchronous call.
        let written = unsafe {
            ksp_product_edit_render_page_region(
                self.handle.as_ptr(),
                page_index,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                full_width as i32,
                full_height as i32,
                rgba.as_mut_ptr(),
                capacity as i32,
            )
        };
        if written < 0 {
            Err(ProductError::Native(written))
        } else if written != capacity as i32 {
            Err(ProductError::InvalidOutput)
        } else {
            Ok(rgba)
        }
    }
}

impl Drop for ProductSession {
    fn drop(&mut self) {
        // SAFETY: the unique handle is closed only here.
        unsafe { ksp_product_edit_close(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_and_exports_a_real_pdf() {
        let source = include_bytes!("../../harness/assets/editable-two-page.pdf");
        let session = ProductSession::open(source).unwrap();
        assert_eq!(session.page_count(), Ok(2));
        let (width, height) = session.page_size(0).unwrap();
        assert!(width > 0.0 && height > 0.0);
        let chars = session.read_page_text(1).unwrap();
        assert!(!chars.is_empty());
        assert!(chars
            .iter()
            .all(|ch| ch.left.is_finite() && ch.top.is_finite()));
        assert!(matches!(
            session.read_page_text(99),
            Err(ProductError::Native(_))
        ));
        let full = session.render_page(1, 320, 480).unwrap();
        assert_eq!(full.len(), 320 * 480 * 4);
        let tile = session.render_region(1, 53, 71, 128, 96, 320, 480).unwrap();
        for row in 0..96usize {
            assert_eq!(
                &tile[row * 128 * 4..(row + 1) * 128 * 4],
                &full[((row + 71) * 320 + 53) * 4..((row + 71) * 320 + 53 + 128) * 4],
            );
        }
        assert!(matches!(
            session.render_page(99, 320, 480),
            Err(ProductError::Native(_))
        ));
        let snapshot = session.snapshot().unwrap();
        let reopened = ProductSession::open(&snapshot).unwrap();
        assert_eq!(reopened.page_count(), Ok(2));
    }

    #[test]
    fn rejects_invalid_documents() {
        assert!(matches!(
            ProductSession::open(&[]),
            Err(ProductError::EmptyInput)
        ));
        assert!(matches!(
            ProductSession::open(b"not a PDF"),
            Err(ProductError::Native(_))
        ));
    }

    #[test]
    fn reads_links_from_the_product_session() {
        let source = include_bytes!("../../harness/assets/product-links.pdf");
        let session = ProductSession::open(source).unwrap();
        let links = session.read_page_links(0).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(
            links[0].target,
            ProductLinkTarget::Uri(b"https://example.com/pdf".to_vec())
        );
        assert_eq!(links[1].target, ProductLinkTarget::Page(1));
        assert!(session.read_page_links(1).unwrap().is_empty());
        assert!(matches!(
            session.read_page_links(99),
            Err(ProductError::Native(_))
        ));
    }
}
