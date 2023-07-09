import * as React from "react";
import styled from "styled-components";
import steamIcon from "./icons/steam.png";
import epicIcon from "./icons/epic.png";
import { Provider as ProviderType } from "../../../services/auth_service";
import * as AuthService from "../../../services/auth_service";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import useSession from "../../../hooks/use_session";
import { UserPermissions } from "../../../context/session_context";

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
  WaitingForAuthResponse,
  Failed,
}

const Login = () => {
  const [state, setState] = React.useState(LoginState.WaitingForLoginOption);
  const [error, setError] = React.useState<string | null>(null);
  const [searchParams, setSearchParams] = useSearchParams();
  const session = useSession();
  const navigate = useNavigate();

  const login = async (provider: ProviderType) => {
    setState(LoginState.WaitingForAuthResponse);
    let response = await AuthService.login(provider);

    if (response.error != null) {
      setState(LoginState.Failed);
      setError(response.error);
      return;
    }

    window.location.href = response.data!.redirectUrl;
  };

  React.useEffect(() => {
    // If we're in the initial state check if callback_token query parameter is present
    if (state == LoginState.WaitingForLoginOption) {
      let callbackToken = searchParams.get("callback_token");

      if (callbackToken == null) {
        return;
      }

      setState(LoginState.WaitingForAuthResponse);

      (async () => {
        var checkResponse = await AuthService.checkAuthResult(callbackToken);

        if (
          checkResponse.error != null ||
          checkResponse.data?.failure != null
        ) {
          if (checkResponse.error != null) {
            setError(checkResponse.error);
          } else {
            setError(checkResponse.data!.failure!.error);
          }

          setState(LoginState.Failed);
        } else if (checkResponse.data?.success != null) {
          const data = checkResponse.data.success;

          session.setSession(
            {
              name: data.name,
              avatarUrl: data.avatarUrl,
              permissions: UserPermissions.None,
            },
            data.authToken
          );
          navigate("/", { replace: true });
        }
      })();
    }
  }, [state]);

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
      case LoginState.WaitingForAuthResponse: {
        return <Content>Waiting for login response...</Content>;
      }
      case LoginState.Failed: {
        return (
          <Content>
            <div>
              <p>Failed to login:</p>
              <p>{error}</p>
              <Link to="/login" reloadDocument={true}>
                Try again
              </Link>
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
