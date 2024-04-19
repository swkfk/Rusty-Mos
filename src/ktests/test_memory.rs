#[cfg(ktest_item = "memory")]
pub fn test_page(
    page_free_list: &mut crate::kern::pmap::PageList,
    pages: &mut *mut crate::kern::pmap::PageNode,
    freemem: &mut usize,
    npage: usize,
) {
    let pp0 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    let pp1 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    let pp2 = crate::kern::pmap::page_alloc(page_free_list, pages).unwrap();
    assert!(!pp0.is_null());
    assert!(!pp1.is_null() && pp1 != pp0);
    assert!(!pp2.is_null() && pp2 != pp0);

    let mut zfl = crate::kern::pmap::page_init(pages, freemem, npage);
    assert!(crate::kern::pmap::page_alloc(&mut zfl, pages).is_err());
}
