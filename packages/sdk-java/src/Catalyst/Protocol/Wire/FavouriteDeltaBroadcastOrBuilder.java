// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: Wire.proto

package Catalyst.Protocol.Wire;

public interface FavouriteDeltaBroadcastOrBuilder extends
    // @@protoc_insertion_point(interface_extends:Catalyst.Protocol.Wire.FavouriteDeltaBroadcast)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   * The candidate favoured by the voter broadcasting this message.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Wire.CandidateDeltaBroadcast candidate = 1;</code>
   */
  boolean hasCandidate();
  /**
   * <pre>
   * The candidate favoured by the voter broadcasting this message.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Wire.CandidateDeltaBroadcast candidate = 1;</code>
   */
  Catalyst.Protocol.Wire.CandidateDeltaBroadcast getCandidate();
  /**
   * <pre>
   * The candidate favoured by the voter broadcasting this message.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Wire.CandidateDeltaBroadcast candidate = 1;</code>
   */
  Catalyst.Protocol.Wire.CandidateDeltaBroadcastOrBuilder getCandidateOrBuilder();

  /**
   * <pre>
   * The PeerId of the participant submitting its vote to the network.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Peer.PeerId voter_id = 2;</code>
   */
  boolean hasVoterId();
  /**
   * <pre>
   * The PeerId of the participant submitting its vote to the network.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Peer.PeerId voter_id = 2;</code>
   */
  Catalyst.Protocol.Peer.PeerId getVoterId();
  /**
   * <pre>
   * The PeerId of the participant submitting its vote to the network.
   * </pre>
   *
   * <code>.Catalyst.Protocol.Peer.PeerId voter_id = 2;</code>
   */
  Catalyst.Protocol.Peer.PeerIdOrBuilder getVoterIdOrBuilder();
}
