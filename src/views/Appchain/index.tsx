import React, { useState, useEffect, useCallback } from "react";

import { useParams } from 'react-router-dom';
import { Card, Descriptions, message, Table, Button } from "antd";
import { LeftOutlined, RightOutlined } from "@ant-design/icons";

import TokenBadge from "../../components/TokenBadge";

import Big from 'big.js';

function Appchain(): React.ReactElement {
  const { id } = useParams();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  
  const [appchain, setAppchain] = useState<any>();

  const [isLoadingValidators, setIsLoadingValidators] = useState<boolean>(true);
  const [currValidatorSetIdx, setCurrValidatorSetIdx] = useState<number>(0);
  const [appchainValidatorIdex, setAppchainValidatorIdx] = useState<number>(0);
  const [validatorSet, setValidatorSet] = useState<any>();

  const columns = [
    {
      title: "Account",
      dataIndex: "account_id",
    },
    {
      title: "Appchain Validator Id",
      dataIndex: "id",
    },
    {
      title: "Weight",
      dataIndex: "weight",
      render: (value) => {
        return (
          <span>{value}</span>
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
      setCurrValidatorSetIdx(idx);
      setAppchainValidatorIdx(idx);
      // getValidators(appchainId, idx);
    });
  }, [id]);

  const getValidators = function(appchaiId, idx) {
    setIsLoadingValidators(true);
    window.contract.get_validator_set({ appchain_id: appchaiId, index: idx })
      .then(set => {
        setIsLoadingValidators(false);
        setValidatorSet(set);
      })
      .catch(err => {
        setIsLoadingValidators(false);
        message.error(err.toString());
      });
  }

  useEffect(() => {
    if (!appchain) return;
    getValidators(appchain.id, currValidatorSetIdx);
  }, [appchain, currValidatorSetIdx]);

  const onPrevIndex = useCallback(() => {
    if (currValidatorSetIdx > 0) {
      setCurrValidatorSetIdx(currValidatorSetIdx - 1);
    }
  }, [currValidatorSetIdx]);

  const onNextIndex = useCallback(() => {
    if (!appchain) return;
    if (currValidatorSetIdx < appchainValidatorIdex) {
      setCurrValidatorSetIdx(currValidatorSetIdx + 1);
    }
  }, [currValidatorSetIdx, appchain]);

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
            <Descriptions.Item label="Bond Tokens">{appchain.bond_tokens} <TokenBadge /></Descriptions.Item>
            <Descriptions.Item label="Status">{appchain.status}</Descriptions.Item>
          </Descriptions>
        }
      </Card>
      <div style={{marginTop: "15px"}}>
        <Card title={<span>Validators 
          <Button type="text" disabled={currValidatorSetIdx <= 0} size="small" icon={<LeftOutlined />} onClick={onPrevIndex} /> 
            Index: {currValidatorSetIdx} <Button size="small" type="text" onClick={onNextIndex} disabled={currValidatorSetIdx >= appchainValidatorIdex} 
            icon={<RightOutlined />} /></span>} 
            loading={isLoading || isLoadingValidators}>
          <Table columns={columns} rowKey={record => record.id} dataSource={validatorSet?.validators} pagination={false} />
        </Card>
      </div>
    </div>
  );
}

export default React.memo(Appchain);