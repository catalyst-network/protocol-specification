<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: DfsMarketplace.proto

namespace Catalyst\Protocol\DfsMarketplace;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Protobuf type <code>Catalyst.Protocol.DfsMarketplace.BlockChallengeBroadcast</code>
 */
class BlockChallengeBroadcast extends \Google\Protobuf\Internal\Message
{
    /**
     * <code>.Catalyst.Protocol.DfsMarketplace.BlockChallengeRequest original_challenge = 1;</code>
     */
    private $original_challenge = null;
    /**
     * <code>string answer = 2;</code>
     */
    private $answer = '';
    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_peer = 3;</code>
     */
    private $challenged_peer = null;
    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_by = 4;</code>
     */
    private $challenged_by = null;

    public function __construct() {
        \GPBMetadata\DfsMarketplace::initOnce();
        parent::__construct();
    }

    /**
     * <code>.Catalyst.Protocol.DfsMarketplace.BlockChallengeRequest original_challenge = 1;</code>
     */
    public function getOriginalChallenge()
    {
        return $this->original_challenge;
    }

    /**
     * <code>.Catalyst.Protocol.DfsMarketplace.BlockChallengeRequest original_challenge = 1;</code>
     */
    public function setOriginalChallenge(&$var)
    {
        GPBUtil::checkMessage($var, \Catalyst\Protocol\DfsMarketplace\BlockChallengeRequest::class);
        $this->original_challenge = $var;
    }

    /**
     * <code>string answer = 2;</code>
     */
    public function getAnswer()
    {
        return $this->answer;
    }

    /**
     * <code>string answer = 2;</code>
     */
    public function setAnswer($var)
    {
        GPBUtil::checkString($var, True);
        $this->answer = $var;
    }

    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_peer = 3;</code>
     */
    public function getChallengedPeer()
    {
        return $this->challenged_peer;
    }

    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_peer = 3;</code>
     */
    public function setChallengedPeer(&$var)
    {
        GPBUtil::checkMessage($var, \Catalyst\Protocol\Peer\PeerId::class);
        $this->challenged_peer = $var;
    }

    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_by = 4;</code>
     */
    public function getChallengedBy()
    {
        return $this->challenged_by;
    }

    /**
     * <code>.Catalyst.Protocol.Peer.PeerId challenged_by = 4;</code>
     */
    public function setChallengedBy(&$var)
    {
        GPBUtil::checkMessage($var, \Catalyst\Protocol\Peer\PeerId::class);
        $this->challenged_by = $var;
    }

}

