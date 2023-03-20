+++
title = "Installation"
date = 2019-11-27
weight = 10
template = "docs-page.html"
+++


This is a step by step guide to installing the theme.

### Step 1: Create a new zola site

```bash
zola init mysite
```

### Step 2: Install The Tech Startup Theme

Download this theme to your themes directory:

```bash
cd mysite/themes
git clone https://github.com/purton-tech/tech-startup-theme.git
```

Or install as a submodule:

```bash
cd mysite
git init  # if your project is a git repository already, ignore this command
git submodule add https://github.com/purton-tech/tech-startup-theme.git themes/tech-startup-theme
```

### Step 3: Configuration

Enable the theme in your `config.toml` in the site directory:

```toml
theme = "tech-startup-theme"
```

Or copy the `config.toml.example` from the theme directory to your project's
root directory:

```bash
cp themes/tech-startup-theme/config.toml config.toml
```

### Step 4: Add new content

You can copy the content from the theme directory to your project:

```bash
cp -r themes/tech-startup-theme/content .
```

You can modify or add new posts in the `content/blog`, `content/docs` or other
content directories as needed.

### Step 5: Run the project

Just run `zola serve` in the root path of the project:

```bash
zola serve
```

This will start the Zola development web server accessible by default at
`http://127.0.0.1:1111`. Saved changes will live reload in the browser.

## Customisation

You can customize your configurations, templates and content for yourself. Look
at the `config.toml`, `theme.toml`, `content` files and templates files in this
repo for an idea.