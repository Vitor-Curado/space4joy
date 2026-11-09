export function setupDropdown(element: HTMLElement) {
  const button = element.querySelector<HTMLButtonElement>(
    ".dropdown-button",
  );

  const menu = element.querySelector<HTMLElement>(".dropdown-menu");

  if (!button || !menu) {
    return;
  }

  const close = () => {
    element.classList.remove("open");
    button.setAttribute("aria-expanded", "false");
  };

  const open = () => {
    element.classList.add("open");
    button.setAttribute("aria-expanded", "true");
  };

  const handleClick = () => {
    const isOpen = element.classList.contains("open");

    // Close other dropdowns.
    document.querySelectorAll<HTMLElement>("[data-dropdown].open")
      .forEach((dropdown) => {
        if (dropdown !== element) {
          dropdown.classList.remove("open");

          dropdown
            .querySelector(".dropdown-button")
            ?.setAttribute("aria-expanded", "false");
        }
      });

    if (isOpen) {
      close();
    } else {
      open();
    }
  };

  const handleOutsideClick = (event: MouseEvent) => {
    if (!element.contains(event.target as Node)) {
      close();
    }
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      close();
      button.focus();
    }
  };

  button.addEventListener("click", handleClick);
  document.addEventListener("click", handleOutsideClick);
  element.addEventListener("keydown", handleKeydown);

  return () => {
    button.removeEventListener("click", handleClick);
    document.removeEventListener("click", handleOutsideClick);
    element.removeEventListener("keydown", handleKeydown);
  };
}