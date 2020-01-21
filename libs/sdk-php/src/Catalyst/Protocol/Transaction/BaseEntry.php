<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: Transaction.proto

namespace Catalyst\Protocol\Transaction;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Protobuf type <code>Catalyst.Protocol.Transaction.BaseEntry</code>
 */
class BaseEntry extends \Google\Protobuf\Internal\Message
{
    /**
     * <pre>
     * A nonce, similar to Ethereum, incremented on each transaction on the account issuing the transaction
     * </pre>
     *
     * <code>uint64 nonce = 1;</code>
     */
    private $nonce = 0;
    /**
     * <pre>
     * PublicKey of receiver.
     * </pre>
     *
     * <code>bytes receiver_public_key = 2;</code>
     */
    private $receiver_public_key = '';
    /**
     * <pre>
     * PublicKey of sender.
     * </pre>
     *
     * <code>bytes sender_public_key = 3;</code>
     */
    private $sender_public_key = '';
    /**
     * <pre>
     * 8 bytes, clear text, fees * 10^12
     * </pre>
     *
     * <code>bytes transaction_fees = 4;</code>
     */
    private $transaction_fees = '';

    public function __construct() {
        \GPBMetadata\Transaction::initOnce();
        parent::__construct();
    }

    /**
     * <pre>
     * A nonce, similar to Ethereum, incremented on each transaction on the account issuing the transaction
     * </pre>
     *
     * <code>uint64 nonce = 1;</code>
     */
    public function getNonce()
    {
        return $this->nonce;
    }

    /**
     * <pre>
     * A nonce, similar to Ethereum, incremented on each transaction on the account issuing the transaction
     * </pre>
     *
     * <code>uint64 nonce = 1;</code>
     */
    public function setNonce($var)
    {
        GPBUtil::checkUint64($var);
        $this->nonce = $var;
    }

    /**
     * <pre>
     * PublicKey of receiver.
     * </pre>
     *
     * <code>bytes receiver_public_key = 2;</code>
     */
    public function getReceiverPublicKey()
    {
        return $this->receiver_public_key;
    }

    /**
     * <pre>
     * PublicKey of receiver.
     * </pre>
     *
     * <code>bytes receiver_public_key = 2;</code>
     */
    public function setReceiverPublicKey($var)
    {
        GPBUtil::checkString($var, False);
        $this->receiver_public_key = $var;
    }

    /**
     * <pre>
     * PublicKey of sender.
     * </pre>
     *
     * <code>bytes sender_public_key = 3;</code>
     */
    public function getSenderPublicKey()
    {
        return $this->sender_public_key;
    }

    /**
     * <pre>
     * PublicKey of sender.
     * </pre>
     *
     * <code>bytes sender_public_key = 3;</code>
     */
    public function setSenderPublicKey($var)
    {
        GPBUtil::checkString($var, False);
        $this->sender_public_key = $var;
    }

    /**
     * <pre>
     * 8 bytes, clear text, fees * 10^12
     * </pre>
     *
     * <code>bytes transaction_fees = 4;</code>
     */
    public function getTransactionFees()
    {
        return $this->transaction_fees;
    }

    /**
     * <pre>
     * 8 bytes, clear text, fees * 10^12
     * </pre>
     *
     * <code>bytes transaction_fees = 4;</code>
     */
    public function setTransactionFees($var)
    {
        GPBUtil::checkString($var, False);
        $this->transaction_fees = $var;
    }

}

