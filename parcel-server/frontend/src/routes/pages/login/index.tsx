import * as React from "react";
import styled from "styled-components";
import steamIcon from "./icons/steam.png";
import epicIcon from "./icons/epic.png";
import {
  InitAuthResponse,
  Provider as ProviderType,
} from "../../../services/auth_service";
import * as AuthService from "../../../services/auth_service";

const Wrapper = styled.div`
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const LoginBox = styled.div`
  padding: 1rem;
  background: rgba(31, 71, 96, 0.5);
  width: 20rem;
  border-radius: 0.1rem;
`;

const Title = styled.div`
  display: flex;
  justify-content: center;
  margin-bottom: 0.5rem;
  font-weight: bold;
`;

const Content = styled.div`
  display: flex;
  justify-content: center;
`;

const Provider = styled.a`
  cursor: pointer;
  padding: 0.25rem;

  & img {
    max-width: 3rem;
    max-height: 3rem;
  }

  transition: scale 0.2s ease-out;

  &:hover {
    scale: 1.1;
  }
`;

enum LoginState {
  WaitingForLoginOption,
  WaitingForInitAuthResponse,
  WaitingForProviderLogin,
  Failed,
}

const Login = () => {
  const [state, setState] = React.useState(LoginState.WaitingForLoginOption);
  const [error, setError] = React.useState<string | null>(null);
  const [initAuthResponse, setInitAuthResponse] =
    React.useState<InitAuthResponse | null>(null);

  const login = async (provider: ProviderType) => {
    setState(LoginState.WaitingForInitAuthResponse);
    let response = await AuthService.login(provider);

    if (response.error != null) {
      setState(LoginState.Failed);
      setError(response.error);
      return;
    }

    setState(LoginState.WaitingForProviderLogin);
    setInitAuthResponse(response.data);

    window.location.href = response.data!.redirectUrl;
  };

  const renderCurrentState = () => {
    switch (state) {
      case LoginState.WaitingForLoginOption: {
        return (
          <Content>
            <Provider onClick={() => login(ProviderType.Steam)}>
              <img src={steamIcon} />
            </Provider>
            <Provider onClick={() => login(ProviderType.Epic)}>
              <img src={epicIcon} />
            </Provider>
          </Content>
        );
      }
      case LoginState.WaitingForInitAuthResponse:
      case LoginState.WaitingForProviderLogin: {
        return <Content>Waiting for login response...</Content>;
      }
      case LoginState.Failed: {
        return (
          <Content>
            <div>
              <p>Failed to login:</p>
              <p>{error}</p>
            </div>
          </Content>
        );
      }
    }
  };

  return (
    <Wrapper>
      <LoginBox>
        <Title>Log in</Title>
        {renderCurrentState()}
      </LoginBox>
    </Wrapper>
  );
};

export default Login;
