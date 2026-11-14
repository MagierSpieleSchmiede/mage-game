import { siteLayout } from "./mathstruct/layout"

export const { sharedPageComponents, defaultContentPageLayout, defaultListPageLayout } =
  siteLayout({
    footerLinks: {
      GitHub: "https://github.com/MagierSpieleSchmiede/mage-game",
    },
    // search only this vault, not other sites
    searchSites: [],
  })
