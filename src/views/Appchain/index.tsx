import React, { useState, useEffect } from "react";

import { useParams } from 'react-router-dom';
import { Card, Descriptions, message, Table } from "antd";

import Big from 'big.js';

function Appchain(): React.ReactElement {
  const { id } = useParams();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  
  const [appchain, setAppchain] = useState<any>();

  const [isLoadingValidators, setIsLoadingValidators] = useState<boolean>(false);
  const [validators, setValidators] = useState<any>();

  const columns = [
    {
      title: "Validator",
      dataIndex: "account_id",
      key: "account_id"
    },
    {
      title: "Staked Balance",
      dataIndex: "staked_balance",
      key: "staked_balance",
      render: (value) => {
        return (
          <span>{Big(value).div(10 ** 24).toFixed()} Ⓝ</span>
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
    window.contract.get_appchain({ id: appchainId })
      .then(appchain => {
        setIsLoading(false);
        setAppchain(appchain);
        getValidators(appchainId);
      });
  }, [id]);

  const getValidators = function(id) {
    setIsLoadingValidators(true);
    window.contract.get_appchain_validators({ id })
      .then(validators => {
        setIsLoadingValidators(false);
        setValidators(validators);
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
            <Descriptions.Item label="Staked Balance">{Big(appchain.staked_balance).div(10 ** 24).toFixed()} Ⓝ</Descriptions.Item>
            <Descriptions.Item label="Status">{appchain.status}</Descriptions.Item>
          </Descriptions>
        }
      </Card>
      <div style={{marginTop: "15px"}}>
        <Card title="Validators" loading={isLoading || isLoadingValidators}>
          <Table columns={columns} dataSource={validators} pagination={false} />
        </Card>
      </div>
    </div>
  );
}

export default React.memo(Appchain);