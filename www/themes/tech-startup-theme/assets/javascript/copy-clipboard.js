const copyButtonLabel = "Copy";



document.addEventListener("DOMContentLoaded", function (event) {
  // use a class selector if available
  let blocks = document.querySelectorAll("pre");

  blocks.forEach((block) => {
    // only add a button if browser supports Clipboard API
    if (navigator.clipboard) {
      let button = document.createElement("button");
      button.innerText = copyButtonLabel;
      button.classList.add('clipboard');
      button.addEventListener("click", copyCode);
      block.appendChild(button);
    }
  });

  async function copyCode(event) {
    const button = event.srcElement;

    button.innerText = "Copied";

    setTimeout(() => {
      button.innerText = copyButtonLabel;
    }, 5000)

    const pre = button.parentElement;
    let code = pre.querySelector("code");
    let text = code.innerText;
    await navigator.clipboard.writeText(text);
  }
});