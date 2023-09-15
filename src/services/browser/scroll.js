(() => {
  //this script scrolls the page to the bottom to allow
  //rendering of lazy-loaded components
  const scroll = (resolve) => {
    const scrollingElement = document.scrollingElement || document.body;
    scrollingElement.scrollBy(0, 100);
    const tID = setTimeout(() => {
      scroll(resolve);
    }, 100);

    if (scrollingElement.scrollTop / scrollingElement.scrollHeight >= 0.9) {
      clearTimeout(tID);
      resolve();
    }
  };
  return new Promise((resolve) => {
    scroll(resolve);
  });
})();
