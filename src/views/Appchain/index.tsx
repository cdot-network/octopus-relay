import React, { useState, useEffect } from "react";

import { useParams } from 'react-router-dom';
import { Card, Descriptions, message, Table } from "antd";

import TokenBadge from "../../components/TokenBadge";

import Big from 'big.js';

function Appchain(): React.ReactElement {
  const { id } = useParams();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  
  const [appchain, setAppchain] = useState<any>();

  const [isLoadingValidators, setIsLoadingValidators] = useState<boolean>(false);
  const [appchainCurrValidatorSetIdx, setAppchainCurrValidatorSetIdx] = useState<number>(0);
  const [validatorSet, setValidatorSet] = useState<any>();

  const columns = [
    {
      title: "Validator",
      dataIndex: "id",
    },
    {
      title: "Appchain Account",
      dataIndex: "ocw_id",
    },
    {
      title: "Staked Balance",
      dataIndex: "staked_balance",
      key: "staked_balance",
      render: (value) => {
        return (
          <span>{value} <TokenBadge /></span>
        );
      }
    }
  ];

  useEffect(() => {
    setIsLoading(true);
    let appchainId = 0;
    if (!isNaN(id as any)) {
      appchainId = +id;
    }
    Promise.all([
      window.contract.get_appchain({ appchain_id: appchainId }),
      window.contract.get_curr_validator_set_index({ appchain_id: appchainId })
    ]).then(([appchain, idx]) => {
      setIsLoading(false);
      setAppchain(appchain);
      setAppchainCurrValidatorSetIdx(idx);
      getValidators(appchainId, idx);
    });
  }, [id]);

  const getValidators = function(appchaiId, idx) {
    setIsLoadingValidators(true);
    window.contract.get_validator_set({ appchain_id: appchaiId, index: idx })
      .then(set => {
        console.log(set);
        setIsLoadingValidators(false);
        setValidatorSet(set);
      })
      .catch(err => {
        setIsLoadingValidators(false);
        message.error(err.toString());
      });
  }

  return (
    <div>
      <Card loading={isLoading}>
        {
          appchain !== undefined &&
          <Descriptions title="Appchain Info" column={2}>
            <Descriptions.Item label="Appchain Id">{id}</Descriptions.Item>
            <Descriptions.Item label="Appchain Name">{appchain.appchain_name}</Descriptions.Item>
            <Descriptions.Item label="Founder">{appchain.founder_id}</Descriptions.Item>
            <Descriptions.Item label="Runtime">{appchain.runtime_url}</Descriptions.Item>
            <Descriptions.Item label="Bond Balance">{appchain.bond_balance} <TokenBadge /></Descriptions.Item>
            <Descriptions.Item label="Status">{appchain.status}</Descriptions.Item>
          </Descriptions>
        }
      </Card>
      <div style={{marginTop: "15px"}}>
        <Card title={<span>Validators (Validator Set Index: {appchainCurrValidatorSetIdx})</span>} loading={isLoading || isLoadingValidators}>
          <Table columns={columns} rowKey={record => record.id} dataSource={validatorSet?.validators} pagination={false} />
        </Card>
      </div>
    </div>
  );
}

export default React.memo(Appchain);