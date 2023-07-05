import * as React from "react";
import styled from "styled-components";
import steamIcon from "./icons/steam.png";
import epicIcon from "./icons/epic.png";

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
  font-weight: bold;
`;

const Title = styled.div`
  display: flex;
  justify-content: center;
  margin-bottom: 0.5rem;
`;

const LoginMethods = styled.div`
  display: flex;
  justify-content: center;
`;

const Provider = styled.a`
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

const Login = () => {
  return (
    <Wrapper>
      <LoginBox>
        <Title>Log in</Title>
        <LoginMethods>
          <Provider href="#">
            <img src={steamIcon} />
          </Provider>
          <Provider href="#">
            <img src={epicIcon} />
          </Provider>
        </LoginMethods>
      </LoginBox>
    </Wrapper>
  );
};

export default Login;
