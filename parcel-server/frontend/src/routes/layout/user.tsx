import * as React from "react";
import { User } from "../../context/session_context";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

const Wrapper = styled.div`
  padding: 0 1rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;

  & .name {
    font-weight: bold;
    font-size: 0.8rem;
  }

  & .avatar {
    height: 2rem;
    width: 2rem;
    width: 100%;
    border-radius: 50%;
    border: 1px solid ${Colors.tealDark.teal11};
    box-shadow: 0 0 1px 0px ${Colors.tealDark.teal11} inset,
      0 0 1px 0px ${Colors.tealDark.teal11};
  }
`;

interface Props {
  user: User;
}

const User = ({ user }: Props) => {
  return (
    <Wrapper>
      <span className="name">{user.name}</span>
      <img className="avatar" src={user.avatarUrl} alt="avatar" />
    </Wrapper>
  );
};

export default User;
