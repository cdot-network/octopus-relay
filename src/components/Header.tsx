import React, { useCallback } from "react";

import { Button, Menu, Dropdown } from "antd";
import { CaretDownOutlined, UserOutlined } from "@ant-design/icons";

import styled from "styled-components";

import logo from "../assets/logo.png";

const Wrapper = styled.div`
  background: #fff;
  border-bottom: 1px solid #e7e8ea;
  .container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 15px 20px;
    align-items: center;
  }
  .logo {
    display: inline-block;
    width: 150px;
    img {
      display: block;
      width: 100%;
      height: auto;
    }
  }
`;

import { login, logout } from "../utils";

const menu = (
  <Menu>
    <Menu.Item onClick={logout}>
      Sign Out
    </Menu.Item>
  </Menu>
);

function Header(): React.ReactElement {

  return (
    <Wrapper>
      <div className="container">
        <div className="left">
          <a className="logo" href=".">
            <img src={logo} />
          </a>
        </div>
        <div className="right">
          {
            window.walletConnection?.isSignedIn() ?
            <Dropdown overlay={menu}>
              <div>
                <span><UserOutlined /> { window.accountId }</span>
                <span style={{ marginLeft: "5px", color: "#9c9c9c" }}>
                  <CaretDownOutlined />
                </span>
              </div>
            </Dropdown> :
            <Button type="primary" onClick={login}><UserOutlined /> Sign In</Button>
          }
          
        </div>
      </div>
    </Wrapper>
  );
}

export default React.memo(Header);