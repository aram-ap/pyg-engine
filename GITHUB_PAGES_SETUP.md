# GitHub Pages Setup Guide

This guide explains how the GitHub Pages site is configured and how to maintain it.

## Configuration Files

### `_config.yml`
- Main Jekyll configuration file
- Defines site metadata, navigation, and build settings
- Uses the Cayman theme for a clean, professional look

### `Gemfile`
- Specifies Ruby dependencies for Jekyll
- Includes necessary plugins for SEO, sitemap, and feeds

### `.github/workflows/jekyll-gh-pages.yml`
- GitHub Actions workflow for automatic deployment
- Builds the site and deploys to GitHub Pages on push to main branch

## Site Structure

```
pyg_engine/
├── _config.yml              # Jekyll configuration
├── _layouts/                # Custom layouts
│   └── default.html        # Main layout template
├── _includes/              # Reusable components
│   └── head-custom.html    # Custom CSS and head content
├── docs/                   # Documentation
│   └── CORE_SYSTEMS_GUIDE.md
├── index.md                # Homepage
├── examples.md             # Examples page
├── Gemfile                 # Ruby dependencies
└── .github/workflows/      # GitHub Actions
    └── jekyll-gh-pages.yml
```

## Customization

### Updating Site Information
Edit `_config.yml` to update:
- Site title and description
- GitHub username and repository URLs
- Navigation links
- Footer links

### Adding New Pages
1. Create a new `.md` file in the root directory
2. Add Jekyll front matter at the top:
   ```yaml
   ---
   layout: default
   title: Your Page Title
   permalink: /your-page-url/
   ---
   ```
3. Add navigation link in `_config.yml`

### Styling Changes
- Edit `_includes/head-custom.html` for CSS changes
- The site uses the Cayman theme as a base

## Deployment

The site automatically deploys when you push to the main branch. The workflow:

1. Sets up Ruby and Jekyll environment
2. Installs dependencies from Gemfile
3. Builds the site using Jekyll
4. Deploys to GitHub Pages

## Local Development

To test the site locally:

1. Install Ruby and Bundler
2. Run `bundle install`
3. Run `bundle exec jekyll serve`
4. Visit `http://localhost:4000`

## Troubleshooting

### Common Issues

1. **Build fails**: Check that all required files exist and have proper front matter
2. **Navigation not working**: Verify URLs in `_config.yml` are correct
3. **Styling issues**: Check CSS in `head-custom.html`

### Updating Dependencies

To update Jekyll or theme versions:
1. Update versions in `Gemfile`
2. Run `bundle update`
3. Test locally with `bundle exec jekyll serve`
4. Commit and push changes

## Notes

- The site uses GitHub Pages' automatic Jekyll support
- All documentation is written in Markdown
- Images should be placed in an `images/` directory
- Code examples are syntax-highlighted automatically 