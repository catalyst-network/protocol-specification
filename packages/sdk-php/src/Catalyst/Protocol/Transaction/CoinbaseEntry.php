<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: Transaction.proto

namespace Catalyst\Protocol\Transaction;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Protobuf type <code>Catalyst.Protocol.Transaction.CoinbaseEntry</code>
 */
class CoinbaseEntry extends \Google\Protobuf\Internal\Message
{
    /**
     * <pre>
     * public key behind the address where the Coinbase will be credited
     * </pre>
     *
     * <code>bytes receiver_public_key = 1;</code>
     */
    private $receiver_public_key = '';
    /**
     * <pre>
     * uint256 amount
     * </pre>
     *
     * <code>bytes amount = 2;</code>
     */
    private $amount = '';

    public function __construct() {
        \GPBMetadata\Transaction::initOnce();
        parent::__construct();
    }

    /**
     * <pre>
     * public key behind the address where the Coinbase will be credited
     * </pre>
     *
     * <code>bytes receiver_public_key = 1;</code>
     */
    public function getReceiverPublicKey()
    {
        return $this->receiver_public_key;
    }

    /**
     * <pre>
     * public key behind the address where the Coinbase will be credited
     * </pre>
     *
     * <code>bytes receiver_public_key = 1;</code>
     */
    public function setReceiverPublicKey($var)
    {
        GPBUtil::checkString($var, False);
        $this->receiver_public_key = $var;
    }

    /**
     * <pre>
     * uint256 amount
     * </pre>
     *
     * <code>bytes amount = 2;</code>
     */
    public function getAmount()
    {
        return $this->amount;
    }

    /**
     * <pre>
     * uint256 amount
     * </pre>
     *
     * <code>bytes amount = 2;</code>
     */
    public function setAmount($var)
    {
        GPBUtil::checkString($var, False);
        $this->amount = $var;
    }

}

