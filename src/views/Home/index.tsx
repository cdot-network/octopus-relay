import React, { useCallback, useEffect, useState } from "react";
import { Row, Col, Button, Table, Card, message, Statistic } from "antd";

import { PlusOutlined } from "@ant-design/icons";

import { utils } from 'near-api-js';
import { Link } from "react-router-dom";

import Big from 'big.js';
import RegisterModal from "./RegisterModal";
import StakingModal from "./StakingModal";

import TokenBadge from "../../components/TokenBadge";
import Status from "../../components/Status";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 14).toFixed();

function Home(): React.ReactElement {

  let isSignedIn = window.walletConnection?.isSignedIn();

  const [isLoadingList, setIsLoadingList] = useState<boolean>(false);
  const [isLoadingOverview, setIsLoadingOverview] = useState<boolean>(false);

  const [registerModalVisible, setRegisterModalVisible] = useState<boolean>(false);
  const [stakingModalVisible, setStakingModalVisible] = useState<boolean>(false);

  const [numberAppchains, setNumberAppchains] = useState<number>(0);
  const [miniumStakingAmount, setMiniumStakingAmount] = useState<number>(0);
  const [stakedBalance, setStakedBalance] = useState<number>(0);
  const [totalBalance, setTotalBalance] = useState<number>(0);

  const [appchains, setAppchains] = useState<any[]>();

  const [activing, setActiving] = useState<boolean>(false);

  const [appchainId, setAppchainId] = useState<number>(0);

  const columns = [
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
        const { validators } = fields;
        return <span>{validators.length}</span>
      }
    },
    {
      title: "Bonded",
      dataIndex: "bond_tokens",
      render: (value) => {
        return (
          <span>
            { value } <TokenBadge />
          </span>
        )
      }
    },
    {
      title: "Status",
      dataIndex: "status",
      render: (text) => {
        return (
          <Status type={text} />
        );
      }
    },
    {
      title: "Action",
      key: "action",
      render: (text, fields) => {
        const { id, validators, founder_id, status } = fields;
        return (
          <div>
            {
              window.accountId &&
              (
                window.accountId == founder_id ?
                (
                  status == "Frozen" && 
                  <Button type="primary" onClick={() => activeAppchain(fields.id)} loading={activing}>Active</Button>
                ) :
                <Button onClick={() => { setAppchainId(fields.id); toggleStakingModalVisible(); }} type="link">Staking</Button>
             
              )
            }
            <span style={{ marginLeft: '10px' }}><Link to={`/appchain/${id}`}>Detail</Link></span>
          </div>
        );
      }
    }
  ];

  const toggleRegisterModalVisible = useCallback(() => {
    setRegisterModalVisible(!registerModalVisible);
  }, [registerModalVisible]);

  const toggleStakingModalVisible = useCallback(() => {
    setStakingModalVisible(!stakingModalVisible);
  }, [stakingModalVisible]);

  const getAppchains = useCallback(() => {
    setIsLoadingList(true);
    setIsLoadingOverview(true);
    Promise.all([
        window.contract.get_num_appchains(),
        window.contract.get_total_staked_balance(),
        window.contract.get_minium_staking_amount(),
        window.tokenContract.get_balance({ owner_id: window.contractName })
      ])
      .then(([num_appchains, stakedBlance, amount, balance]) => {
        setIsLoadingOverview(false);
        setNumberAppchains(num_appchains);
        setMiniumStakingAmount(amount);
        setStakedBalance(stakedBlance);
        setTotalBalance(balance);
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

  const [isFetching, setIsFetching] = useState<boolean>(false);
  const [currBlock, setCurrBlock] = useState<number>(0);

  // initialize
  useEffect(() => {
    getAppchains();

    let timer = setInterval(() => {
      if (isFetching) return false;
      setIsFetching(true);
      utils.web.fetchJson(window.walletConnection._near?.config.nodeUrl, JSON.stringify({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block", 
        "params": {
            "finality": "final"
        }
      })).then(({ result }) => {
        setCurrBlock(result.header.height);
      }).finally(() => {
        setIsFetching(false);
      });
    }, 1000);

    return () => {
      clearInterval(timer)
    };

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

  const activeAppchain = function(appchainId) {
    setActiving(true);
    window.contract.active_appchain(
      {
        appchain_id: appchainId,
      },
      BOATLOAD_OF_GAS,
      0
    ).then(() => {
      setActiving(false);
      window.location.reload();
    }).catch((err) => {
      setActiving(false);
      message.error(err.toString());
    });
  }

  return (
    <>
     <Card title="Overview">
        <Row gutter={16}>
          <Col span={8}>
            <Statistic title="Total Appchains" value={numberAppchains} />
          </Col>
          <Col span={8}>
            <Statistic title="Staked / Total Balance"  value={stakedBalance} suffix={<span>/{totalBalance} <TokenBadge /></span>} />
          </Col>
          <Col span={8}>
            <Statistic title="Block Height" value={currBlock} />
          </Col>
        </Row>
      </Card>
      <div style={{ marginTop: "15px" }}>
        <Card title="Appchains" extra={
          isSignedIn &&
          <Button type="primary" onClick={toggleRegisterModalVisible} icon={<PlusOutlined />}>Register</Button>
        }>
          <Table rowKey={(record) => record.id} columns={columns} loading={isLoadingList} dataSource={appchains} />
        </Card>
      </div>
      <RegisterModal visible={registerModalVisible} onCancel={toggleRegisterModalVisible} onOk={onRegister} />
      <StakingModal appchainId={appchainId} visible={stakingModalVisible} onCancel={toggleStakingModalVisible} />
    </>
  );
}

export default React.memo(Home);