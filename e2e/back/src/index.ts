import { accountPlugin } from './plugins/account/account.plugin';
import { decorateWithUser } from './authentication.decorator';
import { initFastify } from './config/fastify.config';
import { decorateWithDatabase } from './database/database.decorator';
import { profilePlugin } from './plugins/profile/profile.plugin';
import { loginPlugin } from './plugins/login/login.plugin';
import { userPlugin } from './plugins/user/user.plugin';
import { securityPlugin } from './plugins/security/security.plugin';

const fastify = initFastify(
  [decorateWithUser, decorateWithDatabase],
  [
    { plugin: profilePlugin, routesPrefix: '/profiles' },
    { plugin: accountPlugin, routesPrefix: '/accounts' },
    { plugin: loginPlugin, routesPrefix: '/login' },
    { plugin: userPlugin, routesPrefix: '/user' },
    { plugin: securityPlugin, routesPrefix: '' }
  ]
);

const start = async () => {
  try {
    await fastify.listen({ port: 3000 })
  } catch (err) {
    fastify.log.error(err)
    process.exit(1)
  }
}
start();