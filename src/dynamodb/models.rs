use serde::{Deserialize, Serialize};

/// <p>Represents the data for an attribute.</p> <p>Each attribute value is described as a name-value pair. The name is the data type, and the value is the data itself.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes">Data Types</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AttributeValue {
    /// <p>An attribute of type Binary. For example:</p> <p> <code>"B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"</code> </p>
    #[serde(rename = "B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::dynamodb::serializers::serialize", 
        deserialize_with = "crate::dynamodb::serializers::deserialize"
    )]
    pub b: Option<bytes::Bytes>,
    /// <p>An attribute of type Boolean. For example:</p> <p> <code>"BOOL": true</code> </p>
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// <p>An attribute of type Binary Set. For example:</p> <p> <code>"BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]</code> </p>
    #[serde(rename = "BS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        serialize_with = "crate::dynamodb::serializers::serialize_list",
        deserialize_with = "crate::dynamodb::serializers::deserialize_list"
    )]
    pub bs: Option<Vec<bytes::Bytes>>,
    /// <p>An attribute of type List. For example:</p> <p> <code>"L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N", "3.14159"}]</code> </p>
    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    /// <p>An attribute of type Map. For example:</p> <p> <code>"M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}</code> </p>
    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>An attribute of type Number. For example:</p> <p> <code>"N": "123.45"</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// <p>An attribute of type Number Set. For example:</p> <p> <code>"NS": ["42.2", "-19", "7.5", "3.14"]</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    /// <p>An attribute of type Null. For example:</p> <p> <code>"NULL": true</code> </p>
    #[serde(rename = "NULL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    /// <p>An attribute of type String. For example:</p> <p> <code>"S": "Hello"</code> </p>
    #[serde(rename = "S")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// <p>An attribute of type String Set. For example:</p> <p> <code>"SS": ["Giraffe", "Hippo" ,"Zebra"]</code> </p>
    #[serde(rename = "SS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<Vec<String>>,
}

/// <p>Represents the amount of provisioned throughput capacity consumed on a table or an index.</p>
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Capacity {
    /// <p>The total number of capacity units consumed on a table or an index.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    /// <p>The total number of read capacity units consumed on a table or an index.</p>
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<f64>,
    /// <p>The total number of write capacity units consumed on a table or an index.</p>
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<f64>,
}

/// <p>The capacity units consumed by an operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the request asked for it. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ConsumedCapacity {
    /// <p>The total number of capacity units consumed by the operation.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    /// <p>The amount of throughput consumed on each global index affected by the operation.</p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<::std::collections::HashMap<String, Capacity>>,
    /// <p>The amount of throughput consumed on each local index affected by the operation.</p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<::std::collections::HashMap<String, Capacity>>,
    /// <p>The total number of read capacity units consumed by the operation.</p>
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<f64>,
    /// <p>The amount of throughput consumed on the table affected by the operation.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Capacity>,
    /// <p>The name of the table that was affected by the operation.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The total number of write capacity units consumed by the operation.</p>
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<f64>,
}

/// <p>Information about item collections, if any, that were affected by the operation. <code>ItemCollectionMetrics</code> is only returned if the request asked for it. If the table does not have any local secondary indexes, this information is not returned in the response.</p>
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ItemCollectionMetrics {
    /// <p>The partition key value of the item collection. This value is the same as the partition key value of the item.</p>
    #[serde(rename = "ItemCollectionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p>
    #[serde(rename = "SizeEstimateRangeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate_range_gb: Option<Vec<f64>>,
}

/// <p>Represents the output of a <code>PutItem</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PutItemOutput {
    /// <p>The attribute values as they appeared before the <code>PutItem</code> operation, but only if <code>ReturnValues</code> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The capacity units consumed by the <code>PutItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Read/Write Capacity Mode</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p><p>Information about item collections, if any, that were affected by the <code>PutItem</code> operation. <code>ItemCollectionMetrics</code> is only returned if the <code>ReturnItemCollectionMetrics</code> parameter was specified. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <code>ItemCollectionMetrics</code> element consists of:</p> <ul> <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul></p>
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}
