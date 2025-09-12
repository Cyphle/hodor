import './Home.scss';
import {BASE_PATH} from "../../helpers/http.ts";
import {Button} from 'antd';

export const Home = () => {


    const checkMe = () => {
        fetch(`${BASE_PATH}/api/users/me`, {
        })
            .then((response: Response) => {
                return response.json();
            })
            .then(data => {
                console.log(data);
            });
    }

    const logout = () => {
        fetch(`${BASE_PATH}/logout`, {
        })
            .then((response: Response) => {
                return response.json();
            })
            .then(data => {
                console.log(data);
            });
    }

    const testDb = () => {
        fetch(`${BASE_PATH}/test`, {
        })
            .then((response: Response) => {
                return response.json();
            })
            .then(data => {
                console.log(data);
            });
    }

  return (
    <div className="homepage">
      <div className="main-title">
        <h1>Banana</h1>
        <p>Un gestionnaire de compte bancaire facile et familial</p>
      </div>

        <div>
            <Button onClick={handleLogin} type="primary" htmlType="submit">
                Connexion
            </Button>

            <Button onClick={checkMe} type="primary" htmlType="submit">USERS ME</Button>

            <Button onClick={logout} type="primary" htmlType="submit">LOGOUT</Button>

            <Button onClick={testDb} type="primary" htmlType="submit">TEST database</Button>
        </div>
    </div>
  )
}