// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: Cryptography.proto

package Catalyst.Protocol.Cryptography;

public final class Cryptography {
  private Cryptography() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
  }

  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistry registry) {
    registerAllExtensions(
        (com.google.protobuf.ExtensionRegistryLite) registry);
  }
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_Catalyst_Protocol_Cryptography_Signature_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_Catalyst_Protocol_Cryptography_Signature_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_Catalyst_Protocol_Cryptography_SigningContext_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_Catalyst_Protocol_Cryptography_SigningContext_fieldAccessorTable;
  static final com.google.protobuf.Descriptors.Descriptor
    internal_static_Catalyst_Protocol_Cryptography_SignatureBatch_descriptor;
  static final 
    com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internal_static_Catalyst_Protocol_Cryptography_SignatureBatch_fieldAccessorTable;

  public static com.google.protobuf.Descriptors.FileDescriptor
      getDescriptor() {
    return descriptor;
  }
  private static  com.google.protobuf.Descriptors.FileDescriptor
      descriptor;
  static {
    java.lang.String[] descriptorData = {
      "\n\022Cryptography.proto\022\036Catalyst.Protocol." +
      "Cryptography\032\rNetwork.proto\"g\n\tSignature" +
      "\022G\n\017signing_context\030\001 \001(\0132..Catalyst.Pro" +
      "tocol.Cryptography.SigningContext\022\021\n\traw" +
      "_bytes\030\002 \001(\014\"\225\001\n\016SigningContext\022<\n\014netwo" +
      "rk_type\030\001 \001(\0162&.Catalyst.Protocol.Networ" +
      "k.NetworkType\022E\n\016signature_type\030\002 \001(\0162-." +
      "Catalyst.Protocol.Cryptography.Signature" +
      "Type\"\\\n\016SignatureBatch\022\022\n\nsignatures\030\001 \003" +
      "(\014\022\023\n\013public_keys\030\002 \003(\014\022\020\n\010messages\030\003 \003(" +
      "\014\022\017\n\007context\030\004 \001(\014*\230\001\n\rSignatureType\022\032\n\026" +
      "SIGNATURE_TYPE_UNKNOWN\020\000\022\026\n\022TRANSACTION_" +
      "PUBLIC\020\001\022\034\n\030TRANSACTION_CONFIDENTIAL\020\002\022\020" +
      "\n\014PROTOCOL_RPC\020\003\022\021\n\rPROTOCOL_PEER\020\004\022\020\n\014W" +
      "EB3_MESSAGE\020\005*\222\002\n\tErrorCode\022\026\n\022ERROR_COD" +
      "E_UNKNOWN\020\000\022\025\n\021INVALID_SIGNATURE\020\001\022\026\n\022IN" +
      "VALID_PUBLIC_KEY\020\002\022\027\n\023INVALID_PRIVATE_KE" +
      "Y\020\003\022\"\n\036SIGNATURE_VERIFICATION_FAILURE\020\004\022" +
      "\032\n\026INVALID_CONTEXT_LENGTH\020\005\022\031\n\025INVALID_B" +
      "ATCH_MESSAGE\020\006\022\033\n\027ARRAYS_NOT_EQUAL_LENGT" +
      "H\020\007\022\036\n\032BATCH_VERIFICATION_FAILURE\020\010\022\r\n\010N" +
      "O_ERROR\020\242\003B\002P\001b\006proto3"
    };
    descriptor = com.google.protobuf.Descriptors.FileDescriptor
      .internalBuildGeneratedFileFrom(descriptorData,
        new com.google.protobuf.Descriptors.FileDescriptor[] {
          Catalyst.Protocol.Network.Network.getDescriptor(),
        });
    internal_static_Catalyst_Protocol_Cryptography_Signature_descriptor =
      getDescriptor().getMessageTypes().get(0);
    internal_static_Catalyst_Protocol_Cryptography_Signature_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_Catalyst_Protocol_Cryptography_Signature_descriptor,
        new java.lang.String[] { "SigningContext", "RawBytes", });
    internal_static_Catalyst_Protocol_Cryptography_SigningContext_descriptor =
      getDescriptor().getMessageTypes().get(1);
    internal_static_Catalyst_Protocol_Cryptography_SigningContext_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_Catalyst_Protocol_Cryptography_SigningContext_descriptor,
        new java.lang.String[] { "NetworkType", "SignatureType", });
    internal_static_Catalyst_Protocol_Cryptography_SignatureBatch_descriptor =
      getDescriptor().getMessageTypes().get(2);
    internal_static_Catalyst_Protocol_Cryptography_SignatureBatch_fieldAccessorTable = new
      com.google.protobuf.GeneratedMessageV3.FieldAccessorTable(
        internal_static_Catalyst_Protocol_Cryptography_SignatureBatch_descriptor,
        new java.lang.String[] { "Signatures", "PublicKeys", "Messages", "Context", });
    Catalyst.Protocol.Network.Network.getDescriptor();
  }

  // @@protoc_insertion_point(outer_class_scope)
}
