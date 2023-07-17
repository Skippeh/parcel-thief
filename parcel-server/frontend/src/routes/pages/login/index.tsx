import * as React from "react";
import styled from "styled-components";
import steamIcon from "./icons/steam.png";
import epicIcon from "./icons/epic.png";
import { Provider as ProviderType } from "../../../api_types";
import * as AuthService from "../../../services/auth_service";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import useSession from "../../../hooks/use_session";
import { UserPermissions } from "../../../context/session_context";
import * as Tabs from "../../../components/tabs";
import Footer from "../../layout/footer";

const Wrapper = styled.div`
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const Title = styled.div`
  font-weight: bold;
  margin-bottom: 2rem;
  text-align: center;
`;

const LoginBox = styled.div`
  background: rgba(31, 71, 96, 0.5);
  width: 20rem;
  height: 250px;
  border-radius: 4px;
`;

const Content = styled.div`
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
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

const TabsRoot = styled(Tabs.Root)`
  height: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 0fr 1fr;
`;

const TabsList = styled(Tabs.List)`
  grid-area: 1 / 1 / 1 / 1;
  display: flex;
  justify-content: stretch;
  font-size: 1.1rem;
`;

const TabsTrigger = styled(Tabs.Trigger)`
  flex: 1;
  text-align: center;
`;

const TabsContent = styled(Tabs.Content)`
  grid-area: 2 / 1 / 2 / 1;
  height: 100%;
  padding: 1rem;
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

        if (checkResponse.error != null) {
          setError(checkResponse.error);
          setState(LoginState.Failed);
        } else if (checkResponse.data?.type == "failure") {
          setError(checkResponse.data.error);
          setState(LoginState.Failed);
        } else if (checkResponse.data?.type == "success") {
          const data = checkResponse.data;

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
          <TabsRoot defaultValue="provider">
            <TabsList>
              <TabsTrigger value="provider">Provider</TabsTrigger>
              <TabsTrigger value="account">Account</TabsTrigger>
            </TabsList>
            <TabsContent value="provider">
              <Content>
                <Provider onClick={() => login("steam")}>
                  <img src={steamIcon} />
                </Provider>
                <Provider onClick={() => login("epic")}>
                  <img src={epicIcon} />
                </Provider>
              </Content>
            </TabsContent>
            <TabsContent value="account">Account login</TabsContent>
          </TabsRoot>
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
              <Link to="/login" reloadDocument>
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
      <div>
        <Title>Log in</Title>
        <LoginBox>{renderCurrentState()}</LoginBox>
        <Footer />
      </div>
    </Wrapper>
  );
};

export default Login;
