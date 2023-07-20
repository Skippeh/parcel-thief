import * as React from "react";
import { User } from "../../context/session_context";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";
import * as DropdownMenu from "../../components/dropdown_menu";
import useSession from "../../hooks/use_session";
import { useNavigate } from "react-router-dom";

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
  const { logout } = useSession();
  const navigate = useNavigate();

  function doLogout() {
    logout();
    navigate("/login");
  }

  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <Wrapper>
          <span className="name">{user.name}</span>
          {user.avatarUrl && (
            <img className="avatar" src={user.avatarUrl} alt="avatar" />
          )}
        </Wrapper>
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content align="end">
          <DropdownMenu.Item onClick={doLogout}>Log out</DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
};

export default User;
