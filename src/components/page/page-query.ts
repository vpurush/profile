import { contentPanelQueryFragment } from "../content-panel/content-panel-query";

export const pageQuery = `
query($slug: String!) {
  pageCollection (preview: true, where: {slug: $slug}) {
    items {
      title
      slug
      panelsCollection {
        items {
          __typename
          ${contentPanelQueryFragment}
        }
      }
    }
  }
}
`;
