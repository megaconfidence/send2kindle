(() => {
  //this script scrolls the page to the bottom to allow
  //rendering of lazy-loaded components. Auto stops after 30s
  let tid;
  const scroll = (resolve) => {
    const scrollingElement = document.scrollingElement || document.body;
    scrollingElement.scrollBy(0, 100);
    tid = setTimeout(() => scroll(resolve), 100);

    const scrollPercentage =
      scrollingElement.scrollTop / scrollingElement.scrollHeight;
    if (scrollPercentage >= 0.9) {
      clearTimeout(tid);
      resolve();
    }
  };
  return new Promise((resolve) => {
    const hasScrollBar =
      window.innerWidth > document.documentElement.clientWidth;
    if (hasScrollBar) {
      scroll(resolve);
      //auto stop after 15s
      setTimeout(() => {
        clearTimeout(tid);
        resolve();
      }, 15000);
    } else {
      resolve();
    }
  });
})();
