+++
title = "Configuration"
weight = 20
template = "docs-page.html"
+++

The `[extra]` section of the `config.toml` files allows you to configure certains arts of the theme.

## config.toml



```toml
[extra]
copyright = "Copyright 2021 - 2022"

# Every page CTA. At the bottom of every blog post we have a cta
every_page_cta = { title = "Your 30 seconds away from trying us out", cta_url = "/blog", cta_button = "Sign Up" }

# On the footer we have a signup button and a call to action, set the text here
footer_cta = { title = "Sign up for our service", prompt = "Get control of your secrets and add security.", cta_url = "/blog", cta_button = "Check it Out" }

# Where shall we point the sign in and sign up buttons.
authentication = { sign_up = "/blog", sign_in = "/blog" }
# When running on localhost you can set the sign up and sign links to somewhere else.
localhost = "http://localhost:1111"
authentication_development = { sign_up = "/blog", sign_in = "/blog" }

# Information required by terms and privacy pages.
support_email = "support@tech-startup.com"
company_address = "71-75 Shelton Street, London, LND, WC2H 9JQ, United Kingdom"

# Social icons (shown on the footer)
github_repo = "https://github.com/purton-tech/tech-startup-theme"
twitter = "https://twitter.com/ianpurton"
linkedin = "https://www.linkedin.com/in/ianpurton/"

first_doc_page = "/docs/getting-started/introduction/"

# Resources on the footer (LHS)
resources_1 = [
    {name = "Blog", url = "/blog"},
    {name = "Documentation", url = "/docs/getting-started/introduction"},
    {name = "FAQ", url = "/blog"},
]

resources_2 = [
    {name = "Pricing", url = "/pricing"},
    {name = "Mission", url = "/blog"},
    {name = "Careers", url = "/blog"},
]

menu_items = [
    {name = "Pricing", url = "/pricing"},
]

# Documentation sections
doc_sections = [
    {name = "Reference", location = "reference"},
]
```