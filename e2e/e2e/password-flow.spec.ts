import test, { expect } from '@playwright/test';
import { getTokenUrl, IDP_CONFIG } from '../config/idp';
import { form } from '../helpers/form';

test('should forge an OAuth2 access token using password flow with a confidential client', async ({ request }) => {
    const tokenRes = await request.post(getTokenUrl(), {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
          Authorization: 'Basic ' + Buffer.from(`${IDP_CONFIG.client_id}:${IDP_CONFIG.client_secret}`).toString('base64')
        },
        data: form({
          grant_type: 'password',
          username: IDP_CONFIG.username,
          password: IDP_CONFIG.password,
          scope: 'profile email'
        })
      });
  
      expect(tokenRes.ok(), 'token endpoint should return 200').toBeTruthy();
  
      const tokenBody = await tokenRes.json();
      expect(tokenBody).toHaveProperty('access_token');
      expect(tokenBody).toHaveProperty('token_type', 'Bearer');
      expect(tokenBody).toHaveProperty('expires_in');
      expect(tokenBody).toHaveProperty('scope', 'profile email');
});
