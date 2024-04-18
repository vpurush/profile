import {
  CONTENTFUL_DELIVERY_URL,
} from "~/utils/constant";
import { prefixSlash } from "~/utils/prefixSlash";
import { ContentfulPage } from "./types";

const query = `
  query($slug: String!) {
    pageCollection (preview: true, where: {slug: $slug}) {
      items {
        title
        slug
      }
    }
  }
`;

export const getPage = (slug: string): Promise<ContentfulPage | undefined> => {
  return fetch(CONTENTFUL_DELIVERY_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${import.meta.env.VITE_CONTENTFUL_PREVIEW_TOKEN}`,
    },
    body: JSON.stringify({
      query,
      variables: {
        'slug': prefixSlash(slug),
      },
    }),
  })
    .then((response) => {
      return response.json();
    })
    .then((jsonResponse) => {
      console.log("jsonResponse", jsonResponse);
      return jsonResponse?.data?.pageCollection?.items?.[0];
    });
};
