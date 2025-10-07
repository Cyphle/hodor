import { env } from 'process';

export const IDP_CONFIG = {
    idp_url: env.IDP_URL ?? 'http://localhost:8080',
    client_id: env.CLIENT_ID ?? 'test',
    client_secret: env.CLIENT_SECRET ?? 'test',
    username: 'johndoe',
    password: 'johndoepassword',
    authorize_path: '/oauth/authorize',
    token_path: '/oauth/token'
}

export const getTokenUrl = (): string => `${IDP_CONFIG.idp_url}${IDP_CONFIG.token_path}`;
