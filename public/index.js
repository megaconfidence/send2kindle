const form = document.querySelector("form");
const infoText = document.querySelector(".info-text");
const infoButton = document.querySelector(".info-button");
const info = document.querySelector(".info");

init();
function init() {
  form.elements["email"].value = localStorage.getItem("email");
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();
  form.classList.toggle("hide");
  info.classList.toggle("hide");

  const url = form.elements["url"].value;
  const email = form.elements["email"].value;
  const isRemeberEmail = form.elements["remember-email"].checked;
  shouldRememberEmail(isRemeberEmail, email);

  const payload = { url, email };
  const res = await makeApiCall(payload);
  infoText.textContent = res.message;
  infoButton.textContent = res.status == 200 ? "Send Another" : "Fix Error";
});

infoButton.addEventListener("click", () => {
  form.classList.toggle("hide");
  info.classList.toggle("hide");
  init();
});

function shouldRememberEmail(isRemember, email) {
  if (isRemember) {
    localStorage.setItem("email", email);
  } else {
    localStorage.removeItem("email");
  }
}

async function makeApiCall(payload) {
  const response = await fetch("/send", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  const message = (await response.json()).message;
  const status = response.status;

  return { message, status };
}
