import {
  CONTENTFUL_DELIVERY_URL,
} from "~/utils/constant";
import { prefixSlash } from "~/utils/prefixSlash";
import { pageQuery } from "./page-query";
import { ContentfulPage } from "./types";


export const getPage = (slug: string): Promise<ContentfulPage | undefined> => {
  return fetch(CONTENTFUL_DELIVERY_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${import.meta.env.VITE_CONTENTFUL_PREVIEW_TOKEN}`,
    },
    body: JSON.stringify({
      query: pageQuery,
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
