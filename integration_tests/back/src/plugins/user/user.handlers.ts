import { Database } from '../../database/database';
import { Profile } from '../profile/profile.types';
import { UserInfo } from './user.types';

export const userInfoByUsernameHandler = (database: Database) => (username: string): UserInfo | undefined => {
  const userInfo = database.readOneByField<Profile>('profiles', 'username', username);
  if (!!userInfo) {
    return { ...userInfo };
  } else {
    return undefined;
  }
}
