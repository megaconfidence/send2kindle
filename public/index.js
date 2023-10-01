let apiBase = "";
let apiPath = "/send";
const browser = window?.browser || window?.chrome;

const form = document.querySelector("form");
const info = document.querySelector(".info");
const infoText = document.querySelector(".info-text");
const infoButton = document.querySelector(".info-button");

init();
function init() {
  form.elements["email"].value = localStorage.getItem("email");
  // check if app is in extension mode
  if (browser?.extension) {
    document.querySelector("body").style.width = "300px";
    document.querySelector(".home").classList.add("hide");
    document.querySelector(".divider").classList.add("hide");
    document.querySelector(".app h2 em").textContent = "Send2Kindle⚡";

    // auto-fill window address
    browser.tabs.query({ currentWindow: true, active: true }, (tabs) => {
      form.elements["url"].value = tabs[0].url;
    });
    apiBase = "https://send2kindle.confidence.sh";
  }
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();
  form.classList.toggle("hide");
  info.classList.toggle("hide");

  const url = form.elements["url"].value;
  const email = form.elements["email"].value;
  const shouldRemeberEmail = form.elements["remember-email"].checked;
  shouldRememberEmail(shouldRemeberEmail, email);

  const payload = { url, email };
  const res = await makeApiCall(payload);
  infoText.textContent = res.message;
  infoButton.textContent = res.status == 200 ? "Send Another" : "Retry";
});

infoButton.addEventListener("click", () => {
  form.classList.toggle("hide");
  info.classList.toggle("hide");
  init();
});

function shouldRememberEmail(shouldRemember, email) {
  if (shouldRemember) {
    localStorage.setItem("email", email);
  } else {
    localStorage.removeItem("email");
  }
}

async function makeApiCall(payload) {
  try {
    const response = await fetch(apiBase + apiPath, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
    const message = (await response.json()).message;
    const status = response.status;
    return { message, status };
  } catch (e) {
    return { message: "⚠️ " + e.message, status: 400 };
  }
}
