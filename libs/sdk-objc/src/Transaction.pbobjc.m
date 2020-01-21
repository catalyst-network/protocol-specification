// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: Transaction.proto

// This CPP symbol can be defined to use imports that match up to the framework
// imports needed when using CocoaPods.
#if !defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS)
 #define GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS 0
#endif

#if GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
 #import <Protobuf/GPBProtocolBuffers_RuntimeSupport.h>
#else
 #import "GPBProtocolBuffers_RuntimeSupport.h"
#endif

#if GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
 #import <Protobuf/Timestamp.pbobjc.h>
#else
 #import "google/protobuf/Timestamp.pbobjc.h"
#endif

 #import "Transaction.pbobjc.h"
 #import "Cryptography.pbobjc.h"
// @@protoc_insertion_point(imports)

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

#pragma mark - TransactionRoot

@implementation TransactionRoot

// No extensions in the file and none of the imports (direct or indirect)
// defined extensions, so no need to generate +extensionRegistry.

@end

#pragma mark - TransactionRoot_FileDescriptor

static GPBFileDescriptor *TransactionRoot_FileDescriptor(void) {
  // This is called by +initialize so there is no need to worry
  // about thread safety of the singleton.
  static GPBFileDescriptor *descriptor = NULL;
  if (!descriptor) {
    GPB_DEBUG_CHECK_RUNTIME_VERSIONS();
    descriptor = [[GPBFileDescriptor alloc] initWithPackage:@"Catalyst.Protocol.Transaction"
                                                     syntax:GPBFileSyntaxProto3];
  }
  return descriptor;
}

#pragma mark - Enum TransactionType

GPBEnumDescriptor *TransactionType_EnumDescriptor(void) {
  static GPBEnumDescriptor *descriptor = NULL;
  if (!descriptor) {
    static const char *valueNames =
        "TransactionTypeUnknown\000Public\000Confidenti"
        "al\000";
    static const int32_t values[] = {
        TransactionType_TransactionTypeUnknown,
        TransactionType_Public,
        TransactionType_Confidential,
    };
    GPBEnumDescriptor *worker =
        [GPBEnumDescriptor allocDescriptorForName:GPBNSStringifySymbol(TransactionType)
                                       valueNames:valueNames
                                           values:values
                                            count:(uint32_t)(sizeof(values) / sizeof(int32_t))
                                     enumVerifier:TransactionType_IsValidValue];
    if (!OSAtomicCompareAndSwapPtrBarrier(nil, worker, (void * volatile *)&descriptor)) {
      [worker release];
    }
  }
  return descriptor;
}

BOOL TransactionType_IsValidValue(int32_t value__) {
  switch (value__) {
    case TransactionType_TransactionTypeUnknown:
    case TransactionType_Public:
    case TransactionType_Confidential:
      return YES;
    default:
      return NO;
  }
}

#pragma mark - PublicEntry

@implementation PublicEntry

@dynamic hasBase, base;
@dynamic amount;
@dynamic data_p;
@dynamic hasTimestamp, timestamp;
@dynamic hasSignature, signature;
@dynamic gasPrice;
@dynamic gasLimit;

typedef struct PublicEntry__storage_ {
  uint32_t _has_storage_[1];
  BaseEntry *base;
  NSData *amount;
  NSData *data_p;
  GPBTimestamp *timestamp;
  Signature *signature;
  NSData *gasPrice;
  uint64_t gasLimit;
} PublicEntry__storage_;

// This method is threadsafe because it is initially called
// in +initialize for each subclass.
+ (GPBDescriptor *)descriptor {
  static GPBDescriptor *descriptor = nil;
  if (!descriptor) {
    static GPBMessageFieldDescription fields[] = {
      {
        .name = "base",
        .dataTypeSpecific.className = GPBStringifySymbol(BaseEntry),
        .number = PublicEntry_FieldNumber_Base,
        .hasIndex = 0,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, base),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeMessage,
      },
      {
        .name = "amount",
        .dataTypeSpecific.className = NULL,
        .number = PublicEntry_FieldNumber_Amount,
        .hasIndex = 1,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, amount),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "data_p",
        .dataTypeSpecific.className = NULL,
        .number = PublicEntry_FieldNumber_Data_p,
        .hasIndex = 2,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, data_p),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "timestamp",
        .dataTypeSpecific.className = GPBStringifySymbol(GPBTimestamp),
        .number = PublicEntry_FieldNumber_Timestamp,
        .hasIndex = 3,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, timestamp),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeMessage,
      },
      {
        .name = "signature",
        .dataTypeSpecific.className = GPBStringifySymbol(Signature),
        .number = PublicEntry_FieldNumber_Signature,
        .hasIndex = 4,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, signature),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeMessage,
      },
      {
        .name = "gasPrice",
        .dataTypeSpecific.className = NULL,
        .number = PublicEntry_FieldNumber_GasPrice,
        .hasIndex = 5,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, gasPrice),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "gasLimit",
        .dataTypeSpecific.className = NULL,
        .number = PublicEntry_FieldNumber_GasLimit,
        .hasIndex = 6,
        .offset = (uint32_t)offsetof(PublicEntry__storage_, gasLimit),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeUInt64,
      },
    };
    GPBDescriptor *localDescriptor =
        [GPBDescriptor allocDescriptorForClass:[PublicEntry class]
                                     rootClass:[TransactionRoot class]
                                          file:TransactionRoot_FileDescriptor()
                                        fields:fields
                                    fieldCount:(uint32_t)(sizeof(fields) / sizeof(GPBMessageFieldDescription))
                                   storageSize:sizeof(PublicEntry__storage_)
                                         flags:GPBDescriptorInitializationFlag_None];
    NSAssert(descriptor == nil, @"Startup recursed!");
    descriptor = localDescriptor;
  }
  return descriptor;
}

@end

#pragma mark - ConfidentialEntry

@implementation ConfidentialEntry

@dynamic hasBase, base;
@dynamic pedersenCommitment;
@dynamic hasRangeProof, rangeProof;

typedef struct ConfidentialEntry__storage_ {
  uint32_t _has_storage_[1];
  BaseEntry *base;
  NSData *pedersenCommitment;
  RangeProof *rangeProof;
} ConfidentialEntry__storage_;

// This method is threadsafe because it is initially called
// in +initialize for each subclass.
+ (GPBDescriptor *)descriptor {
  static GPBDescriptor *descriptor = nil;
  if (!descriptor) {
    static GPBMessageFieldDescription fields[] = {
      {
        .name = "base",
        .dataTypeSpecific.className = GPBStringifySymbol(BaseEntry),
        .number = ConfidentialEntry_FieldNumber_Base,
        .hasIndex = 0,
        .offset = (uint32_t)offsetof(ConfidentialEntry__storage_, base),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeMessage,
      },
      {
        .name = "pedersenCommitment",
        .dataTypeSpecific.className = NULL,
        .number = ConfidentialEntry_FieldNumber_PedersenCommitment,
        .hasIndex = 1,
        .offset = (uint32_t)offsetof(ConfidentialEntry__storage_, pedersenCommitment),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "rangeProof",
        .dataTypeSpecific.className = GPBStringifySymbol(RangeProof),
        .number = ConfidentialEntry_FieldNumber_RangeProof,
        .hasIndex = 2,
        .offset = (uint32_t)offsetof(ConfidentialEntry__storage_, rangeProof),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeMessage,
      },
    };
    GPBDescriptor *localDescriptor =
        [GPBDescriptor allocDescriptorForClass:[ConfidentialEntry class]
                                     rootClass:[TransactionRoot class]
                                          file:TransactionRoot_FileDescriptor()
                                        fields:fields
                                    fieldCount:(uint32_t)(sizeof(fields) / sizeof(GPBMessageFieldDescription))
                                   storageSize:sizeof(ConfidentialEntry__storage_)
                                         flags:GPBDescriptorInitializationFlag_None];
    NSAssert(descriptor == nil, @"Startup recursed!");
    descriptor = localDescriptor;
  }
  return descriptor;
}

@end

#pragma mark - BaseEntry

@implementation BaseEntry

@dynamic nonce;
@dynamic receiverPublicKey;
@dynamic senderPublicKey;
@dynamic transactionFees;

typedef struct BaseEntry__storage_ {
  uint32_t _has_storage_[1];
  NSData *receiverPublicKey;
  NSData *senderPublicKey;
  NSData *transactionFees;
  uint64_t nonce;
} BaseEntry__storage_;

// This method is threadsafe because it is initially called
// in +initialize for each subclass.
+ (GPBDescriptor *)descriptor {
  static GPBDescriptor *descriptor = nil;
  if (!descriptor) {
    static GPBMessageFieldDescription fields[] = {
      {
        .name = "nonce",
        .dataTypeSpecific.className = NULL,
        .number = BaseEntry_FieldNumber_Nonce,
        .hasIndex = 0,
        .offset = (uint32_t)offsetof(BaseEntry__storage_, nonce),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeUInt64,
      },
      {
        .name = "receiverPublicKey",
        .dataTypeSpecific.className = NULL,
        .number = BaseEntry_FieldNumber_ReceiverPublicKey,
        .hasIndex = 1,
        .offset = (uint32_t)offsetof(BaseEntry__storage_, receiverPublicKey),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "senderPublicKey",
        .dataTypeSpecific.className = NULL,
        .number = BaseEntry_FieldNumber_SenderPublicKey,
        .hasIndex = 2,
        .offset = (uint32_t)offsetof(BaseEntry__storage_, senderPublicKey),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "transactionFees",
        .dataTypeSpecific.className = NULL,
        .number = BaseEntry_FieldNumber_TransactionFees,
        .hasIndex = 3,
        .offset = (uint32_t)offsetof(BaseEntry__storage_, transactionFees),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
    };
    GPBDescriptor *localDescriptor =
        [GPBDescriptor allocDescriptorForClass:[BaseEntry class]
                                     rootClass:[TransactionRoot class]
                                          file:TransactionRoot_FileDescriptor()
                                        fields:fields
                                    fieldCount:(uint32_t)(sizeof(fields) / sizeof(GPBMessageFieldDescription))
                                   storageSize:sizeof(BaseEntry__storage_)
                                         flags:GPBDescriptorInitializationFlag_None];
    NSAssert(descriptor == nil, @"Startup recursed!");
    descriptor = localDescriptor;
  }
  return descriptor;
}

@end

#pragma mark - CoinbaseEntry

@implementation CoinbaseEntry

@dynamic receiverPublicKey;
@dynamic amount;

typedef struct CoinbaseEntry__storage_ {
  uint32_t _has_storage_[1];
  NSData *receiverPublicKey;
  NSData *amount;
} CoinbaseEntry__storage_;

// This method is threadsafe because it is initially called
// in +initialize for each subclass.
+ (GPBDescriptor *)descriptor {
  static GPBDescriptor *descriptor = nil;
  if (!descriptor) {
    static GPBMessageFieldDescription fields[] = {
      {
        .name = "receiverPublicKey",
        .dataTypeSpecific.className = NULL,
        .number = CoinbaseEntry_FieldNumber_ReceiverPublicKey,
        .hasIndex = 0,
        .offset = (uint32_t)offsetof(CoinbaseEntry__storage_, receiverPublicKey),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "amount",
        .dataTypeSpecific.className = NULL,
        .number = CoinbaseEntry_FieldNumber_Amount,
        .hasIndex = 1,
        .offset = (uint32_t)offsetof(CoinbaseEntry__storage_, amount),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
    };
    GPBDescriptor *localDescriptor =
        [GPBDescriptor allocDescriptorForClass:[CoinbaseEntry class]
                                     rootClass:[TransactionRoot class]
                                          file:TransactionRoot_FileDescriptor()
                                        fields:fields
                                    fieldCount:(uint32_t)(sizeof(fields) / sizeof(GPBMessageFieldDescription))
                                   storageSize:sizeof(CoinbaseEntry__storage_)
                                         flags:GPBDescriptorInitializationFlag_None];
    NSAssert(descriptor == nil, @"Startup recursed!");
    descriptor = localDescriptor;
  }
  return descriptor;
}

@end

#pragma mark - RangeProof

@implementation RangeProof

@dynamic valueCommitmentArray, valueCommitmentArray_Count;
@dynamic bitCommitment;
@dynamic perBitBlindingFactorCommitment;
@dynamic polyCommitmentT1;
@dynamic polyCommitmentT2;
@dynamic proofOfShareTau;
@dynamic proofOfShareMu;
@dynamic aggregatedVectorPolynomialLArray, aggregatedVectorPolynomialLArray_Count;
@dynamic aggregatedVectorPolynomialRArray, aggregatedVectorPolynomialRArray_Count;
@dynamic aPrime0;
@dynamic bPrime0;
@dynamic t;

typedef struct RangeProof__storage_ {
  uint32_t _has_storage_[1];
  NSMutableArray *valueCommitmentArray;
  NSData *bitCommitment;
  NSData *perBitBlindingFactorCommitment;
  NSData *polyCommitmentT1;
  NSData *polyCommitmentT2;
  NSData *proofOfShareTau;
  NSData *proofOfShareMu;
  NSMutableArray *aggregatedVectorPolynomialLArray;
  NSMutableArray *aggregatedVectorPolynomialRArray;
  NSData *aPrime0;
  NSData *bPrime0;
  NSData *t;
} RangeProof__storage_;

// This method is threadsafe because it is initially called
// in +initialize for each subclass.
+ (GPBDescriptor *)descriptor {
  static GPBDescriptor *descriptor = nil;
  if (!descriptor) {
    static GPBMessageFieldDescription fields[] = {
      {
        .name = "valueCommitmentArray",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_ValueCommitmentArray,
        .hasIndex = GPBNoHasBit,
        .offset = (uint32_t)offsetof(RangeProof__storage_, valueCommitmentArray),
        .flags = GPBFieldRepeated,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "bitCommitment",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_BitCommitment,
        .hasIndex = 0,
        .offset = (uint32_t)offsetof(RangeProof__storage_, bitCommitment),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "perBitBlindingFactorCommitment",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_PerBitBlindingFactorCommitment,
        .hasIndex = 1,
        .offset = (uint32_t)offsetof(RangeProof__storage_, perBitBlindingFactorCommitment),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "polyCommitmentT1",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_PolyCommitmentT1,
        .hasIndex = 2,
        .offset = (uint32_t)offsetof(RangeProof__storage_, polyCommitmentT1),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "polyCommitmentT2",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_PolyCommitmentT2,
        .hasIndex = 3,
        .offset = (uint32_t)offsetof(RangeProof__storage_, polyCommitmentT2),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "proofOfShareTau",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_ProofOfShareTau,
        .hasIndex = 4,
        .offset = (uint32_t)offsetof(RangeProof__storage_, proofOfShareTau),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "proofOfShareMu",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_ProofOfShareMu,
        .hasIndex = 5,
        .offset = (uint32_t)offsetof(RangeProof__storage_, proofOfShareMu),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "aggregatedVectorPolynomialLArray",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_AggregatedVectorPolynomialLArray,
        .hasIndex = GPBNoHasBit,
        .offset = (uint32_t)offsetof(RangeProof__storage_, aggregatedVectorPolynomialLArray),
        .flags = GPBFieldRepeated,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "aggregatedVectorPolynomialRArray",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_AggregatedVectorPolynomialRArray,
        .hasIndex = GPBNoHasBit,
        .offset = (uint32_t)offsetof(RangeProof__storage_, aggregatedVectorPolynomialRArray),
        .flags = GPBFieldRepeated,
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "aPrime0",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_APrime0,
        .hasIndex = 6,
        .offset = (uint32_t)offsetof(RangeProof__storage_, aPrime0),
        .flags = (GPBFieldFlags)(GPBFieldOptional | GPBFieldTextFormatNameCustom),
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "bPrime0",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_BPrime0,
        .hasIndex = 7,
        .offset = (uint32_t)offsetof(RangeProof__storage_, bPrime0),
        .flags = (GPBFieldFlags)(GPBFieldOptional | GPBFieldTextFormatNameCustom),
        .dataType = GPBDataTypeBytes,
      },
      {
        .name = "t",
        .dataTypeSpecific.className = NULL,
        .number = RangeProof_FieldNumber_T,
        .hasIndex = 8,
        .offset = (uint32_t)offsetof(RangeProof__storage_, t),
        .flags = GPBFieldOptional,
        .dataType = GPBDataTypeBytes,
      },
    };
    GPBDescriptor *localDescriptor =
        [GPBDescriptor allocDescriptorForClass:[RangeProof class]
                                     rootClass:[TransactionRoot class]
                                          file:TransactionRoot_FileDescriptor()
                                        fields:fields
                                    fieldCount:(uint32_t)(sizeof(fields) / sizeof(GPBMessageFieldDescription))
                                   storageSize:sizeof(RangeProof__storage_)
                                         flags:GPBDescriptorInitializationFlag_None];
#if !GPBOBJC_SKIP_MESSAGE_TEXTFORMAT_EXTRAS
    static const char *extraTextFormatInfo =
        "\002\n\001\245\201\000\013\001\245\201\000";
    [localDescriptor setupExtraTextInfo:extraTextFormatInfo];
#endif  // !GPBOBJC_SKIP_MESSAGE_TEXTFORMAT_EXTRAS
    NSAssert(descriptor == nil, @"Startup recursed!");
    descriptor = localDescriptor;
  }
  return descriptor;
}

@end


#pragma clang diagnostic pop

// @@protoc_insertion_point(global_scope)
