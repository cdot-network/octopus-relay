import React, { useCallback, useEffect, useState } from "react";
import { Row, Col, Button, Table, Modal, Form, Input, Card, message, Statistic, Popconfirm, Spin } from "antd";

import { PlusOutlined } from "@ant-design/icons";

import { Link } from "react-router-dom";

import Big from 'big.js';
import RegisterModal from "./RegisterModal";
import StakeModal from "./StakeModal";

import TokenBadge from "../../components/TokenBadge";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 14).toFixed();

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

  const [unstaking, setUnstaking] = useState<boolean>(false);

  const [appchainId, setAppchainId] = useState<number>(0);

  const columns = [
    {
      title: "ID",
      dataIndex: "id",
      render: (text) => {
        return (
          <Link to={`/appchain/${text}`}>{text}</Link>
        );
      }
    },
    {
      title: "Name",
      dataIndex: "appchain_name",
    },
    {
      title: "Founder",
      dataIndex: "founder_id",
    },
    {
      title: "Validators",
      key: "validators",
      render: (_, fields) => {
        const { validator_set, stake_records } = fields;
        let lastValidatorSetIdx = Object.keys(validator_set).pop();
        const vSet = validator_set[lastValidatorSetIdx];
      
        return <span>{vSet.validators.length}/{stake_records.length}</span>
      }
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
      title: "Bonded",
      dataIndex: "bond_tokens",
      render: (value) => {
        return (
          <span>{ value } 
          <TokenBadge />
          </span>
        )
      }
    },
    {
      title: "Status",
      dataIndex: "status"
    },
    {
      title: "Action",
      key: "action",
      render: (text, fields) => {
        const { id, stake_records } = fields;
        
        return (
          <div>
            {
              window.accountId && (
                stake_records.some(r => r.validator.account_id == window.accountId) ?
                <Popconfirm onConfirm={() => unstake(fields.id)} title="Are you sure to unstake?">
                  <Button>Unstake</Button> 
                </Popconfirm>
                :
                <Button onClick={() => {
                  setAppchainId(fields.id);
                  toggleStakeModalVisible();
                }}>Stake</Button>
              )
            }
            <span style={{ marginLeft: '10px' }}><Link to={`/appchain/${id}`}>Detail</Link></span>
            {/* {
              window.accountId && window.accountId == fields.founder_id &&
              <Button type="link" style={{ color: "#f66" }}>Freeze</Button>
            } */}
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
        window.tokenContract.get_balance({
          owner_id: window.contractName
        })
      ])
      .then(([num_appchains, balance]) => {
        setIsLoadingOverview(false);
        setNumberAppchains(num_appchains);
        setStakedBalance(balance);
        return window.contract.get_appchains({from_index: 0, limit: num_appchains});
      })
      .then(list => {
        const t = []
        list.map((item, id) => {
          const t2 = {}
          Object.assign(t2, { id }, item);
          t.push(t2);
        });
     
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
    const { appchainName, runtimeURL, runtimeHash, bondTokenAmount } = values;
    window.contract.register_appchain(
      {
        appchain_name: appchainName,
        runtime_url: runtimeURL,
        runtime_hash: runtimeHash,
        bond_tokens: bondTokenAmount,
      },
      BOATLOAD_OF_GAS,
      Big(3).times(10 ** 22).toFixed(),
    ).then(() => {
      window.location.reload();
    }).catch((err) => {
      message.error(err.toString());
    });

  }

  const onStake = function(values) {
    const { appchainId, validatorId, offchainWorkerId, stakeBalance } = values;
    
    window.contract.stake(
      {
        appchain_id: appchainId,
        id: validatorId,
        ocw_id: offchainWorkerId,
        amount: stakeBalance * 1,
      },
      BOATLOAD_OF_GAS,
      Big(3).times(10 ** 22).toFixed(),
    ).then(() => {
      window.location.reload();
    }).catch((err) => {
      message.error(err.toString());
      setStakeModalVisible(false);
    });
  }

  const unstake = function(appchainId) {
    setUnstaking(true);
    window.contract.unstake(
      {
        appchain_id: appchainId,
      },
      BOATLOAD_OF_GAS,
      0
    ).then(() => {
      setUnstaking(false);
      window.location.reload();
    }).catch((err) => {
      setUnstaking(true);
      message.error(err.toString());
      setStakeModalVisible(false);
    });
  }

  return (
    <>
     <Card title="Overview">
        <Row gutter={16}>
          <Col span={12}>
            <Statistic title="Total Appchains" value={numberAppchains} />
          </Col>
          {/* <Col span={8}>
            <Statistic title="Total Validators" value={numberValidators} />
          </Col> */}
          <Col span={12}>
            <Statistic title="Relay Balance" value={stakedBalance} suffix={<TokenBadge />} />
          </Col>
        </Row>
      </Card>
      <div style={{ marginTop: "15px" }}>
        <Card title="Appchains" extra={
          isSignedIn &&
          <Button type="primary" onClick={toggleRegisterModalVisible} icon={<PlusOutlined />}>Register</Button>
        }>
          <Spin spinning={unstaking}>
            <Table rowKey={(record) => record.id} columns={columns} loading={isLoadingList} dataSource={appchains} />
          </Spin>
        </Card>
      </div>
      <RegisterModal visible={registerModalVisible} onCancel={toggleRegisterModalVisible} onOk={onRegister} />
      <StakeModal appchainId={appchainId} visible={stakeModalVisible} onCancel={toggleStakeModalVisible} onOk={onStake} />
    </>
  );
}

export default React.memo(Home);