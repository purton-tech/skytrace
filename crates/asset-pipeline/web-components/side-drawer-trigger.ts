// We can create a trigger to open drawers

const triggers = () => {
    document.querySelectorAll('[data-drawer-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {
            event.stopImmediatePropagation()
            const attr = row.getAttribute('data-drawer-target');
            if(attr) {
                const drawer = document.getElementById(attr)
                if(drawer) {
                    drawer.setAttribute("open", "true")
                }
            } else {
                console.log("side-drawer-trigger could not find data-drawer-target")
            }
        })
    })
}

document.addEventListener("turbo:load", triggers)
document.addEventListener("turbo:frame-load", triggers)