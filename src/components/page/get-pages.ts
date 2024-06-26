import { CONTENTFUL_DELIVERY_URL } from "~/utils/constant";
import { ContentfulPage } from "./types";

const query = `
  query {
    pageCollection (preview: true) {
      items {
        title
        slug
      }
    }
  }
`;

export const getPageCollection = (): Promise<ContentfulPage[]> => {
  return fetch(CONTENTFUL_DELIVERY_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${import.meta.env.VITE_CONTENTFUL_PREVIEW_TOKEN}`,
    },
    body: JSON.stringify({ query }),
  })
    .then((response) => {
      return response.json();
    })
    .then((jsonResponse) => {
      console.log("jsonResponse", jsonResponse);
      return jsonResponse.data.pageCollection.items;
    });
};
