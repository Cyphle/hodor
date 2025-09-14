import { FastifyInstance, FastifyReply } from 'fastify';
import { Database } from '../../database/database';
import { CustomFastifyRequest } from '../../fastify.types';
import { UserInfo } from './user.types';

export const userInfoController = (handler: (database: Database) => (username: string) => UserInfo | undefined) => (fastify: FastifyInstance): void => {
  fastify.get('/info', (request: CustomFastifyRequest, reply: FastifyReply) => {
    const connectedProfile = request.session.get('user');

    if (connectedProfile === undefined) {
      reply.code(403).send();
    } else {
      const info = handler(request.database!!)(connectedProfile.username);

      if (!!info) {
        reply
        .code(200)
        .header('Content-Type', 'application/json; charset=utf-8')
        .send({ ...info });
      } else {
        reply.code(403).send();
      }
    }
  });
}

export const logoutController = () => (fastify: FastifyInstance): void => {
  fastify.get('/logout', (request: CustomFastifyRequest, reply: FastifyReply) => {
    const connectedProfile = request.session.get('user');
    fastify.log.info(`User ${connectedProfile.username} is login out`);

    if (connectedProfile === undefined) {
      reply.code(403).send();
    } else {
      // @ts-ignore
      request.session.delete('user');
      reply.code(200).send({ message: 'logout success' });
    }
  });
}
