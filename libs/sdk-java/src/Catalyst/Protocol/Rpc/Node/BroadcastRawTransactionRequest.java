// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: Rpc.proto

package Catalyst.Protocol.Rpc.Node;

/**
 * Protobuf type {@code Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest}
 */
public  final class BroadcastRawTransactionRequest extends
    com.google.protobuf.GeneratedMessageV3 implements
    // @@protoc_insertion_point(message_implements:Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)
    BroadcastRawTransactionRequestOrBuilder {
  // Use BroadcastRawTransactionRequest.newBuilder() to construct.
  private BroadcastRawTransactionRequest(com.google.protobuf.GeneratedMessageV3.Builder<?> builder) {
    super(builder);
  }
  private BroadcastRawTransactionRequest() {
  }

  @java.lang.Override
  public final com.google.protobuf.UnknownFieldSet
  getUnknownFields() {
    return com.google.protobuf.UnknownFieldSet.getDefaultInstance();
  }
  private BroadcastRawTransactionRequest(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    this();
    int mutable_bitField0_ = 0;
    try {
      boolean done = false;
      while (!done) {
        int tag = input.readTag();
        switch (tag) {
          case 0:
            done = true;
            break;
          default: {
            if (!input.skipField(tag)) {
              done = true;
            }
            break;
          }
          case 10: {
            Catalyst.Protocol.Wire.TransactionBroadcast.Builder subBuilder = null;
            if (transaction_ != null) {
              subBuilder = transaction_.toBuilder();
            }
            transaction_ = input.readMessage(Catalyst.Protocol.Wire.TransactionBroadcast.parser(), extensionRegistry);
            if (subBuilder != null) {
              subBuilder.mergeFrom(transaction_);
              transaction_ = subBuilder.buildPartial();
            }

            break;
          }
        }
      }
    } catch (com.google.protobuf.InvalidProtocolBufferException e) {
      throw e.setUnfinishedMessage(this);
    } catch (java.io.IOException e) {
      throw new com.google.protobuf.InvalidProtocolBufferException(
          e).setUnfinishedMessage(this);
    } finally {
      makeExtensionsImmutable();
    }
  }
  public static final com.google.protobuf.Descriptors.Descriptor
      getDescriptor() {
    return Catalyst.Protocol.Rpc.Node.Rpc.internal_static_Catalyst_Protocol_Rpc_Node_BroadcastRawTransactionRequest_descriptor;
  }

  protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
      internalGetFieldAccessorTable() {
    return Catalyst.Protocol.Rpc.Node.Rpc.internal_static_Catalyst_Protocol_Rpc_Node_BroadcastRawTransactionRequest_fieldAccessorTable
        .ensureFieldAccessorsInitialized(
            Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.class, Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.Builder.class);
  }

  public static final int TRANSACTION_FIELD_NUMBER = 1;
  private Catalyst.Protocol.Wire.TransactionBroadcast transaction_;
  /**
   * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
   */
  public boolean hasTransaction() {
    return transaction_ != null;
  }
  /**
   * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
   */
  public Catalyst.Protocol.Wire.TransactionBroadcast getTransaction() {
    return transaction_ == null ? Catalyst.Protocol.Wire.TransactionBroadcast.getDefaultInstance() : transaction_;
  }
  /**
   * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
   */
  public Catalyst.Protocol.Wire.TransactionBroadcastOrBuilder getTransactionOrBuilder() {
    return getTransaction();
  }

  private byte memoizedIsInitialized = -1;
  public final boolean isInitialized() {
    byte isInitialized = memoizedIsInitialized;
    if (isInitialized == 1) return true;
    if (isInitialized == 0) return false;

    memoizedIsInitialized = 1;
    return true;
  }

  public void writeTo(com.google.protobuf.CodedOutputStream output)
                      throws java.io.IOException {
    if (transaction_ != null) {
      output.writeMessage(1, getTransaction());
    }
  }

  public int getSerializedSize() {
    int size = memoizedSize;
    if (size != -1) return size;

    size = 0;
    if (transaction_ != null) {
      size += com.google.protobuf.CodedOutputStream
        .computeMessageSize(1, getTransaction());
    }
    memoizedSize = size;
    return size;
  }

  private static final long serialVersionUID = 0L;
  @java.lang.Override
  public boolean equals(final java.lang.Object obj) {
    if (obj == this) {
     return true;
    }
    if (!(obj instanceof Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)) {
      return super.equals(obj);
    }
    Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest other = (Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest) obj;

    boolean result = true;
    result = result && (hasTransaction() == other.hasTransaction());
    if (hasTransaction()) {
      result = result && getTransaction()
          .equals(other.getTransaction());
    }
    return result;
  }

  @java.lang.Override
  public int hashCode() {
    if (memoizedHashCode != 0) {
      return memoizedHashCode;
    }
    int hash = 41;
    hash = (19 * hash) + getDescriptor().hashCode();
    if (hasTransaction()) {
      hash = (37 * hash) + TRANSACTION_FIELD_NUMBER;
      hash = (53 * hash) + getTransaction().hashCode();
    }
    hash = (29 * hash) + unknownFields.hashCode();
    memoizedHashCode = hash;
    return hash;
  }

  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return PARSER.parseFrom(data, extensionRegistry);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseDelimitedWithIOException(PARSER, input, extensionRegistry);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input);
  }
  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageV3
        .parseWithIOException(PARSER, input, extensionRegistry);
  }

  public Builder newBuilderForType() { return newBuilder(); }
  public static Builder newBuilder() {
    return DEFAULT_INSTANCE.toBuilder();
  }
  public static Builder newBuilder(Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest prototype) {
    return DEFAULT_INSTANCE.toBuilder().mergeFrom(prototype);
  }
  public Builder toBuilder() {
    return this == DEFAULT_INSTANCE
        ? new Builder() : new Builder().mergeFrom(this);
  }

  @java.lang.Override
  protected Builder newBuilderForType(
      com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
    Builder builder = new Builder(parent);
    return builder;
  }
  /**
   * Protobuf type {@code Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageV3.Builder<Builder> implements
      // @@protoc_insertion_point(builder_implements:Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)
      Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequestOrBuilder {
    public static final com.google.protobuf.Descriptors.Descriptor
        getDescriptor() {
      return Catalyst.Protocol.Rpc.Node.Rpc.internal_static_Catalyst_Protocol_Rpc_Node_BroadcastRawTransactionRequest_descriptor;
    }

    protected com.google.protobuf.GeneratedMessageV3.FieldAccessorTable
        internalGetFieldAccessorTable() {
      return Catalyst.Protocol.Rpc.Node.Rpc.internal_static_Catalyst_Protocol_Rpc_Node_BroadcastRawTransactionRequest_fieldAccessorTable
          .ensureFieldAccessorsInitialized(
              Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.class, Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.Builder.class);
    }

    // Construct using Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.newBuilder()
    private Builder() {
      maybeForceBuilderInitialization();
    }

    private Builder(
        com.google.protobuf.GeneratedMessageV3.BuilderParent parent) {
      super(parent);
      maybeForceBuilderInitialization();
    }
    private void maybeForceBuilderInitialization() {
      if (com.google.protobuf.GeneratedMessageV3
              .alwaysUseFieldBuilders) {
      }
    }
    public Builder clear() {
      super.clear();
      if (transactionBuilder_ == null) {
        transaction_ = null;
      } else {
        transaction_ = null;
        transactionBuilder_ = null;
      }
      return this;
    }

    public com.google.protobuf.Descriptors.Descriptor
        getDescriptorForType() {
      return Catalyst.Protocol.Rpc.Node.Rpc.internal_static_Catalyst_Protocol_Rpc_Node_BroadcastRawTransactionRequest_descriptor;
    }

    public Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest getDefaultInstanceForType() {
      return Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.getDefaultInstance();
    }

    public Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest build() {
      Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest result = buildPartial();
      if (!result.isInitialized()) {
        throw newUninitializedMessageException(result);
      }
      return result;
    }

    public Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest buildPartial() {
      Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest result = new Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest(this);
      if (transactionBuilder_ == null) {
        result.transaction_ = transaction_;
      } else {
        result.transaction_ = transactionBuilder_.build();
      }
      onBuilt();
      return result;
    }

    public Builder clone() {
      return (Builder) super.clone();
    }
    public Builder setField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        Object value) {
      return (Builder) super.setField(field, value);
    }
    public Builder clearField(
        com.google.protobuf.Descriptors.FieldDescriptor field) {
      return (Builder) super.clearField(field);
    }
    public Builder clearOneof(
        com.google.protobuf.Descriptors.OneofDescriptor oneof) {
      return (Builder) super.clearOneof(oneof);
    }
    public Builder setRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        int index, Object value) {
      return (Builder) super.setRepeatedField(field, index, value);
    }
    public Builder addRepeatedField(
        com.google.protobuf.Descriptors.FieldDescriptor field,
        Object value) {
      return (Builder) super.addRepeatedField(field, value);
    }
    public Builder mergeFrom(com.google.protobuf.Message other) {
      if (other instanceof Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest) {
        return mergeFrom((Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)other);
      } else {
        super.mergeFrom(other);
        return this;
      }
    }

    public Builder mergeFrom(Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest other) {
      if (other == Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest.getDefaultInstance()) return this;
      if (other.hasTransaction()) {
        mergeTransaction(other.getTransaction());
      }
      onChanged();
      return this;
    }

    public final boolean isInitialized() {
      return true;
    }

    public Builder mergeFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws java.io.IOException {
      Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest parsedMessage = null;
      try {
        parsedMessage = PARSER.parsePartialFrom(input, extensionRegistry);
      } catch (com.google.protobuf.InvalidProtocolBufferException e) {
        parsedMessage = (Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest) e.getUnfinishedMessage();
        throw e.unwrapIOException();
      } finally {
        if (parsedMessage != null) {
          mergeFrom(parsedMessage);
        }
      }
      return this;
    }

    private Catalyst.Protocol.Wire.TransactionBroadcast transaction_ = null;
    private com.google.protobuf.SingleFieldBuilderV3<
        Catalyst.Protocol.Wire.TransactionBroadcast, Catalyst.Protocol.Wire.TransactionBroadcast.Builder, Catalyst.Protocol.Wire.TransactionBroadcastOrBuilder> transactionBuilder_;
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public boolean hasTransaction() {
      return transactionBuilder_ != null || transaction_ != null;
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Catalyst.Protocol.Wire.TransactionBroadcast getTransaction() {
      if (transactionBuilder_ == null) {
        return transaction_ == null ? Catalyst.Protocol.Wire.TransactionBroadcast.getDefaultInstance() : transaction_;
      } else {
        return transactionBuilder_.getMessage();
      }
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Builder setTransaction(Catalyst.Protocol.Wire.TransactionBroadcast value) {
      if (transactionBuilder_ == null) {
        if (value == null) {
          throw new NullPointerException();
        }
        transaction_ = value;
        onChanged();
      } else {
        transactionBuilder_.setMessage(value);
      }

      return this;
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Builder setTransaction(
        Catalyst.Protocol.Wire.TransactionBroadcast.Builder builderForValue) {
      if (transactionBuilder_ == null) {
        transaction_ = builderForValue.build();
        onChanged();
      } else {
        transactionBuilder_.setMessage(builderForValue.build());
      }

      return this;
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Builder mergeTransaction(Catalyst.Protocol.Wire.TransactionBroadcast value) {
      if (transactionBuilder_ == null) {
        if (transaction_ != null) {
          transaction_ =
            Catalyst.Protocol.Wire.TransactionBroadcast.newBuilder(transaction_).mergeFrom(value).buildPartial();
        } else {
          transaction_ = value;
        }
        onChanged();
      } else {
        transactionBuilder_.mergeFrom(value);
      }

      return this;
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Builder clearTransaction() {
      if (transactionBuilder_ == null) {
        transaction_ = null;
        onChanged();
      } else {
        transaction_ = null;
        transactionBuilder_ = null;
      }

      return this;
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Catalyst.Protocol.Wire.TransactionBroadcast.Builder getTransactionBuilder() {
      
      onChanged();
      return getTransactionFieldBuilder().getBuilder();
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    public Catalyst.Protocol.Wire.TransactionBroadcastOrBuilder getTransactionOrBuilder() {
      if (transactionBuilder_ != null) {
        return transactionBuilder_.getMessageOrBuilder();
      } else {
        return transaction_ == null ?
            Catalyst.Protocol.Wire.TransactionBroadcast.getDefaultInstance() : transaction_;
      }
    }
    /**
     * <code>.Catalyst.Protocol.Wire.TransactionBroadcast transaction = 1;</code>
     */
    private com.google.protobuf.SingleFieldBuilderV3<
        Catalyst.Protocol.Wire.TransactionBroadcast, Catalyst.Protocol.Wire.TransactionBroadcast.Builder, Catalyst.Protocol.Wire.TransactionBroadcastOrBuilder> 
        getTransactionFieldBuilder() {
      if (transactionBuilder_ == null) {
        transactionBuilder_ = new com.google.protobuf.SingleFieldBuilderV3<
            Catalyst.Protocol.Wire.TransactionBroadcast, Catalyst.Protocol.Wire.TransactionBroadcast.Builder, Catalyst.Protocol.Wire.TransactionBroadcastOrBuilder>(
                getTransaction(),
                getParentForChildren(),
                isClean());
        transaction_ = null;
      }
      return transactionBuilder_;
    }
    public final Builder setUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return this;
    }

    public final Builder mergeUnknownFields(
        final com.google.protobuf.UnknownFieldSet unknownFields) {
      return this;
    }


    // @@protoc_insertion_point(builder_scope:Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)
  }

  // @@protoc_insertion_point(class_scope:Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest)
  private static final Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest DEFAULT_INSTANCE;
  static {
    DEFAULT_INSTANCE = new Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest();
  }

  public static Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static final com.google.protobuf.Parser<BroadcastRawTransactionRequest>
      PARSER = new com.google.protobuf.AbstractParser<BroadcastRawTransactionRequest>() {
    public BroadcastRawTransactionRequest parsePartialFrom(
        com.google.protobuf.CodedInputStream input,
        com.google.protobuf.ExtensionRegistryLite extensionRegistry)
        throws com.google.protobuf.InvalidProtocolBufferException {
        return new BroadcastRawTransactionRequest(input, extensionRegistry);
    }
  };

  public static com.google.protobuf.Parser<BroadcastRawTransactionRequest> parser() {
    return PARSER;
  }

  @java.lang.Override
  public com.google.protobuf.Parser<BroadcastRawTransactionRequest> getParserForType() {
    return PARSER;
  }

  public Catalyst.Protocol.Rpc.Node.BroadcastRawTransactionRequest getDefaultInstanceForType() {
    return DEFAULT_INSTANCE;
  }

}

