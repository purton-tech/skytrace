// We can create a trigger to open drawers

document.addEventListener("turbo:load", function () {
    document.querySelectorAll('[data-popup-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {

            event.stopImmediatePropagation()
            const attr = row.getAttribute('data-popup-target');
            if (attr) {
                const menu = document.getElementById(attr)

                console.log(menu)
                if (menu) {
                    window.addEventListener('click', function (e: MouseEvent) {
                        if (e.target instanceof Element) {
                            if (menu.contains(e.target)) {
                            } else {
                                menu.removeAttribute("active")
                            }
                        }
                    })

                    if (menu.getAttribute("active")) {
                        menu.removeAttribute("active")
                    } else {
                        menu.setAttribute("active", "active")
                    }
                }
            }
        })
    })
})