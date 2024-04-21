import { panelDecorator } from "../panel/panel-decorator";
import { ContentfulPage, PageProps } from "./types";

export const pageDecorator = async (
  contentfulPage: ContentfulPage,
): Promise<PageProps> => {
  return {
    ...contentfulPage,
    panels: contentfulPage.panelsCollection?.items?.map((contentfulPanel) =>
      panelDecorator(contentfulPanel),
    ),
  };
};
