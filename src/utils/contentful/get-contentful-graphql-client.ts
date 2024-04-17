import { ApolloClient, InMemoryCache } from '@apollo/client/core';
import { CONTENTFUL_DELIVERY_URL } from '../constant';

export const getContentfulGraphqlClient = (() => {
    const client = new ApolloClient({
        uri: CONTENTFUL_DELIVERY_URL,
        cache: new InMemoryCache(),
    });

    return () => client
})();