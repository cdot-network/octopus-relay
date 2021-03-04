import React, { useCallback, useEffect, useState } from "react";
import { Row, Col, Button, Table, Modal, Form, Input, Card, message, Statistic } from "antd";

import { PlusOutlined } from "@ant-design/icons";

import { Link } from "react-router-dom";

import Big from 'big.js';
import RegisterModal from "./RegisterModal";
import StakeModal from "./StakeModal";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

function Home(): React.ReactElement {

  let isSignedIn = window.walletConnection?.isSignedIn();

  const [isLoadingList, setIsLoadingList] = useState<boolean>(false);
  const [isLoadingOverview, setIsLoadingOverview] = useState<boolean>(false);

  const [registerModalVisible, setRegisterModalVisible] = useState<boolean>(false);
  const [stakeModalVisible, setStakeModalVisible] = useState<boolean>(false);

  const [numberAppchains, setNumberAppchains] = useState<number>(0);
  const [numberValidators, setNumberValidators] = useState<number>(0);
  const [stakedBalance, setStakedBalance] = useState<number>(0);

  const [appchains, setAppchains] = useState<any[]>();

  const [appchainId, setAppchainId] = useState<number>(0);

  const columns = [
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      render: (text) => {
        return (
          <Link to={`/appchain/${text}`}>{text}</Link>
        );
      }
    },
    {
      title: "Name",
      dataIndex: "appchain_name",
      key: "appchainName"
    },
    {
      title: "Founder",
      dataIndex: "founder_id",
      key: "founderId"
    },
    // {
    //   title: "Runtime",
    //   dataIndex: "runtime_url",
    //   key: "runtimeURL",
    //   render: (text) => {
    //     return (
    //       <div style={{ width: "150px" }}>{text}</div>
    //     );
    //   }
    // },
    {
      title: "Staked",
      dataIndex: "staked_balance",
      key: "stakedBalance",
      render: (value) => {
        return (
          <span>{Big(value).div(10 ** 24).toFixed()} Ⓝ</span>
        )
      }
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status"
    },
    {
      title: "Action",
      dataIndex: "action",
      key: "action",
      render: (text, fields) => {
        return (
          <div>
            <Button onClick={() => {
              setAppchainId(fields.id);
              toggleStakeModalVisible();
            }}>Stake</Button>
            {
              window.accountId && window.accountId == fields.founder_id &&
              <Button type="link" style={{ color: "#f66" }}>Freeze</Button>
            }
          </div>
          
        );
      }
    }
  ];

  const toggleRegisterModalVisible = useCallback(() => {
    setRegisterModalVisible(!registerModalVisible);
  }, [registerModalVisible]);

  const toggleStakeModalVisible = useCallback(() => {
    setStakeModalVisible(!stakeModalVisible);
  }, [stakeModalVisible]);

  const getAppchains = useCallback(() => {
    setIsLoadingList(true);
    setIsLoadingOverview(true);
    Promise.all([
        window.contract.get_num_appchains(),
        window.contract.get_num_validators(),
        window.contract.get_total_staked_balance()
      ])
      .then(([num_appchains, num_validators, staked_balance]) => {
        setIsLoadingOverview(false);
        setNumberAppchains(num_appchains);
        setNumberValidators(num_validators);
        setStakedBalance(Big(staked_balance).div(10 ** 24).toFixed());
        return window.contract.get_appchains({from_index: 0, limit: num_appchains});
      })
      .then(list => {
        const t = []
        list.map((item, id) => {
          const t2 = {}
          Object.assign(t2, { id }, item);
          t.push(t2);
        })
        setAppchains(t);
        setIsLoadingList(false);
      })
      .catch(err => {
        console.log(err);
        message.error(err.toString());
        setIsLoadingList(false);
      })
  }, []);

  // initialize
  useEffect(() => {
    getAppchains();
  }, []);

  const onRegister = function(values) {
    const { appchainName, runtimeURL, runtimeHash, stakeBalance } = values;
   
    window.contract.register_appchain(
      {
        appchain_name: appchainName,
        runtime_url: runtimeURL,
        runtime_hash: runtimeHash,
      },
      BOATLOAD_OF_GAS,
      Big(stakeBalance).times(10 ** 24).toFixed()
    ).then(() => {
      setRegisterModalVisible(false);
    }).catch((err) => {
      message.error(err.toString());
    });

  }

  const onStake = function(values) {
    const { appchainId, appchainAccount, stakeBalance } = values;

    window.contract.stake_to_be_validator(
      {
        appchain_id: appchainId,
        appchain_account: appchainAccount,
      },
      BOATLOAD_OF_GAS,
      Big(stakeBalance).times(10 ** 24).toFixed()
    ).then(() => {
      setStakeModalVisible(false);
    }).catch((err) => {
      message.error(err.toString());
    });
  }

  return (
    <>
     <Card>
        <Row gutter={16}>
          <Col span={8}>
            <Statistic title="Total Appchains" value={numberAppchains} />
          </Col>
          <Col span={8}>
            <Statistic title="Total Validators" value={numberValidators} />
          </Col>
          <Col span={8}>
            <Statistic title="Total Staked Balance" value={stakedBalance} suffix="Ⓝ" />
          </Col>
        </Row>
      </Card>
      <div style={{ marginTop: "15px" }}>
        <Card title="Appchains" extra={
          isSignedIn &&
          <Button type="primary" onClick={toggleRegisterModalVisible} icon={<PlusOutlined />}>Register</Button>
        }>
          <Table columns={columns} loading={isLoadingList} dataSource={appchains} />
        </Card>
      </div>
      <RegisterModal visible={registerModalVisible} onCancel={toggleRegisterModalVisible} onOk={onRegister} />
      <StakeModal appchainId={appchainId} visible={stakeModalVisible} onCancel={toggleStakeModalVisible} onOk={onStake} />
    </>
  );
}

export default React.memo(Home);