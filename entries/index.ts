import "../css/styles.css";
import '@fortawesome/fontawesome-free/js/fontawesome'
import '@fortawesome/fontawesome-free/js/solid'
import '@fortawesome/fontawesome-free/js/regular'
import '@fortawesome/fontawesome-free/js/brands'

declare global {
  interface Window { Popper: any; }
}
window.Popper = window.Popper || {};

window.Popper = require('popper.js').default;

(async () => {
  // Note: files in `crate/pkg/` will be created on the first build.
  await import("../crate/pkg/index");
})();
