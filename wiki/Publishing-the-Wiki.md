# Publishing The Wiki

GitHub Wikis are stored in a separate Git repository. The files in this `wiki/` folder are a starter set you can copy into the project wiki repo.

## 1. Enable The Wiki

In the GitHub repository settings for `STVR393/helius-personal-finance-tracker`, make sure the Wiki feature is enabled.

## 2. Clone The Wiki Repository

```powershell
git clone https://github.com/STVR393/helius-personal-finance-tracker.wiki.git
```

If the wiki was just enabled, GitHub may not create the remote wiki repository until the first wiki page exists. In that case, create a placeholder page in the web UI first, then clone again.

## 3. Copy These Starter Pages

Copy the contents of the local `wiki\` folder into the cloned wiki repo. The important files are:

- `Home.md`
- `_Sidebar.md`
- `Installation.md`
- `Getting-Started.md`
- `CLI-Reference.md`
- `TUI-and-Shell.md`
- `Planning-and-Forecasts.md`
- `Data-and-Storage.md`
- `Development.md`

`Home.md` becomes the landing page.

## 4. Commit And Push

From inside the cloned `.wiki` repository:

```powershell
git add .
git commit -m "Add initial Helius wiki"
git push origin master
```

GitHub Wikis usually use the `master` branch even when the main source repo uses `main`.

## 5. Keep It In Sync

When product behavior changes:

- Update the page in `wiki\` here first
- Copy the changed file into the `.wiki` clone
- Commit and push the wiki repo

## Optional: Create Pages In The Web UI

If you prefer not to use the separate git repo, you can also create pages directly in GitHub's Wiki UI and paste the contents from these markdown files.
