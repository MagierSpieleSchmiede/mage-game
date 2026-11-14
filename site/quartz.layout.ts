import { siteLayout } from "./mathstruct/layout"

export const { sharedPageComponents, defaultContentPageLayout, defaultListPageLayout } =
  siteLayout({
    footerLinks: {
      GitHub: "https://github.com/MagierSpieleSchmiede/mage-game",
    },
    // search only this vault, not other sites
    searchSites: [],
    // a giscus comment thread on every page (GitHub Discussions, category Announcements:
    // only maintainers and the giscus app open threads, everyone can comment).
    // A single page opts out with `comments: false` in its front matter.
    comments: {
      repo: "MagierSpieleSchmiede/mage-game",
      repoId: "R_kgDOVuV_fw",
      category: "Announcements",
      categoryId: "DIC_kwDOVuV_f84DLInO",
    },
  })
