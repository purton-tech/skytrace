// We can create a trigger to open drawers
document.addEventListener("turbo:load", function() {
    document.querySelectorAll('[data-link-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {
            console.log('clicked')
            event.stopImmediatePropagation()
            const attr = row.getAttribute('data-link-target');
            if(attr) {
                window.location.href = attr
            }
        })
    })
})