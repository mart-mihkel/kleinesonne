// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { pub mod public { #[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Breed { Australian,Entlebuch,Bernese,}impl<'a> postgres_types::ToSql for Breed
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { Breed::Australian => "Australian",Breed::Entlebuch => "Entlebuch",Breed::Bernese => "Bernese",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "breed" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Australian" => true,"Entlebuch" => true,"Bernese" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for Breed
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<Breed, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Australian" => Ok(Breed::Australian),"Entlebuch" => Ok(Breed::Entlebuch),"Bernese" => Ok(Breed::Bernese),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "breed" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Australian" => true,"Entlebuch" => true,"Bernese" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Gender { Male,Female,}impl<'a> postgres_types::ToSql for Gender
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { Gender::Male => "Male",Gender::Female => "Female",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "gender" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 2 { return false; }
                variants.iter().all(|v| match &**v
                { "Male" => true,"Female" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for Gender
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<Gender, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Male" => Ok(Gender::Male),"Female" => Ok(Gender::Female),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "gender" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 2 { return false; }
                variants.iter().all(|v| match &**v
                { "Male" => true,"Female" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Availability { Available,Unavailable,Co,}impl<'a> postgres_types::ToSql for Availability
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { Availability::Available => "Available",Availability::Unavailable => "Unavailable",Availability::Co => "Co",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "availability" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Available" => true,"Unavailable" => true,"Co" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for Availability
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<Availability, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Available" => Ok(Availability::Available),"Unavailable" => Ok(Availability::Unavailable),"Co" => Ok(Availability::Co),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "availability" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Available" => true,"Unavailable" => true,"Co" => true,_ => false, })
            } _ => false,
        }
    }
} }}#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod admin
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Admin
{ pub id : i32,pub name : String,pub salt : String,pub hash : String,}pub struct AdminBorrowed<'a> { pub id : i32,pub name : &'a str,pub salt : &'a str,pub hash : &'a str,}
impl<'a> From<AdminBorrowed<'a>> for Admin
{
    fn from(AdminBorrowed { id,name,salt,hash,}: AdminBorrowed<'a>) ->
    Self { Self { id,name: name.into(),salt: salt.into(),hash: hash.into(),} }
}pub struct AdminQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> AdminBorrowed,
    mapper: fn(AdminBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> AdminQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(AdminBorrowed) -> R) ->
    AdminQuery<'a,C,R,N>
    {
        AdminQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn admin_by_name() -> AdminByNameStmt
{ AdminByNameStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name,
	salt,
	hash
FROM
	admins
WHERE
	name = $1")) } pub struct
AdminByNameStmt(cornucopia_async::private::Stmt); impl AdminByNameStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
name: &'a T1,) -> AdminQuery<'a,C,
Admin, 1>
{
    AdminQuery
    {
        client, params: [name,], stmt: &mut self.0, extractor:
        |row| { AdminBorrowed { id: row.get(0),name: row.get(1),salt: row.get(2),hash: row.get(3),} }, mapper: |it| { <Admin>::from(it) },
    }
} }}pub mod dog
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct DogsByBreedAndStatusParams<> { pub breed: super::super::types::public::Breed,pub alive: bool,}#[derive( Debug)] pub struct InsertDogParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::ArraySql<Item = T6>,T8: cornucopia_async::StringSql,T9: cornucopia_async::ArraySql<Item = T8>,T10: cornucopia_async::StringSql,T11: cornucopia_async::ArraySql<Item = T10>,> { pub name: T1,pub nickname: T2,pub father: T3,pub mother: T4,pub breed: super::super::types::public::Breed,pub gender: super::super::types::public::Gender,pub dob: i64,pub alive: bool,pub thumbnail: T5,pub images: T7,pub health: T9,pub titles: T11,}#[derive( Debug)] pub struct UpdateDogParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::ArraySql<Item = T6>,T8: cornucopia_async::StringSql,T9: cornucopia_async::ArraySql<Item = T8>,T10: cornucopia_async::StringSql,T11: cornucopia_async::ArraySql<Item = T10>,> { pub name: T1,pub nickname: T2,pub father: T3,pub mother: T4,pub breed: super::super::types::public::Breed,pub gender: super::super::types::public::Gender,pub dob: i64,pub alive: bool,pub thumbnail: T5,pub images: T7,pub health: T9,pub titles: T11,pub id: i32,}pub struct StringQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> & str,
    mapper: fn(& str) -> T,
} impl<'a, C, T:'a, const N: usize> StringQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(& str) -> R) ->
    StringQuery<'a,C,R,N>
    {
        StringQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Dog
{ pub id : i32,pub name : String,pub nickname : String,pub father : String,pub mother : String,pub breed : super::super::types::public::Breed,pub gender : super::super::types::public::Gender,pub dob : i64,pub alive : bool,pub thumbnail : String,pub images : Vec<String>,pub health : Vec<String>,pub titles : Vec<String>,}pub struct DogBorrowed<'a> { pub id : i32,pub name : &'a str,pub nickname : &'a str,pub father : &'a str,pub mother : &'a str,pub breed : super::super::types::public::Breed,pub gender : super::super::types::public::Gender,pub dob : i64,pub alive : bool,pub thumbnail : &'a str,pub images : cornucopia_async::ArrayIterator<'a, &'a str>,pub health : cornucopia_async::ArrayIterator<'a, &'a str>,pub titles : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<DogBorrowed<'a>> for Dog
{
    fn from(DogBorrowed { id,name,nickname,father,mother,breed,gender,dob,alive,thumbnail,images,health,titles,}: DogBorrowed<'a>) ->
    Self { Self { id,name: name.into(),nickname: nickname.into(),father: father.into(),mother: mother.into(),breed,gender,dob,alive,thumbnail: thumbnail.into(),images: images.map(|v| v.into()).collect(),health: health.map(|v| v.into()).collect(),titles: titles.map(|v| v.into()).collect(),} }
}pub struct DogQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> DogBorrowed,
    mapper: fn(DogBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> DogQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(DogBorrowed) -> R) ->
    DogQuery<'a,C,R,N>
    {
        DogQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn all_names() -> AllNamesStmt
{ AllNamesStmt(cornucopia_async::private::Stmt::new("SELECT
	name
FROM
	dogs")) } pub struct
AllNamesStmt(cornucopia_async::private::Stmt); impl AllNamesStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> StringQuery<'a,C,
String, 0>
{
    StringQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it.into() },
    }
} }pub fn dog_by_id() -> DogByIdStmt
{ DogByIdStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name,
	nickname,
	father,
	mother,
	breed,
	gender,
	dob,
	alive,
	thumbnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	id = $1")) } pub struct
DogByIdStmt(cornucopia_async::private::Stmt); impl DogByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> DogQuery<'a,C,
Dog, 1>
{
    DogQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { DogBorrowed { id: row.get(0),name: row.get(1),nickname: row.get(2),father: row.get(3),mother: row.get(4),breed: row.get(5),gender: row.get(6),dob: row.get(7),alive: row.get(8),thumbnail: row.get(9),images: row.get(10),health: row.get(11),titles: row.get(12),} }, mapper: |it| { <Dog>::from(it) },
    }
} }pub fn dogs_by_breed_and_status() -> DogsByBreedAndStatusStmt
{ DogsByBreedAndStatusStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name,
	nickname,
	father,
	mother,
	breed,
	gender,
	dob,
	alive,
	thumbnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	breed = $1 and alive = $2")) } pub struct
DogsByBreedAndStatusStmt(cornucopia_async::private::Stmt); impl DogsByBreedAndStatusStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
breed: &'a super::super::types::public::Breed,alive: &'a bool,) -> DogQuery<'a,C,
Dog, 2>
{
    DogQuery
    {
        client, params: [breed,alive,], stmt: &mut self.0, extractor:
        |row| { DogBorrowed { id: row.get(0),name: row.get(1),nickname: row.get(2),father: row.get(3),mother: row.get(4),breed: row.get(5),gender: row.get(6),dob: row.get(7),alive: row.get(8),thumbnail: row.get(9),images: row.get(10),health: row.get(11),titles: row.get(12),} }, mapper: |it| { <Dog>::from(it) },
    }
} }impl <'a, C: GenericClient,> cornucopia_async::Params<'a,
DogsByBreedAndStatusParams<>, DogQuery<'a, C,
Dog, 2>, C> for DogsByBreedAndStatusStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    DogsByBreedAndStatusParams<>) -> DogQuery<'a, C,
    Dog, 2>
    { self.bind(client, &params.breed,&params.alive,) }
}pub fn insert_dog() -> InsertDogStmt
{ InsertDogStmt(cornucopia_async::private::Stmt::new("INSERT INTO
	dogs(
		name,
		nickname,
		father,
		mother,
		breed,
		gender,
		dob,
		alive,
		thumbnail,
		images,
		health,
		titles
	)
VALUES(
	$1,
	$2,
	$3,
	$4,
	$5,
	$6,
	$7,
	$8,
	$9,
	$10,
	$11,
	$12)
RETURNING
	id")) } pub struct
InsertDogStmt(cornucopia_async::private::Stmt); impl InsertDogStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::ArraySql<Item = T6>,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::ArraySql<Item = T8>,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::ArraySql<Item = T10>,>(&'a mut self, client: &'a  C,
name: &'a T1,nickname: &'a T2,father: &'a T3,mother: &'a T4,breed: &'a super::super::types::public::Breed,gender: &'a super::super::types::public::Gender,dob: &'a i64,alive: &'a bool,thumbnail: &'a T5,images: &'a T7,health: &'a T9,titles: &'a T11,) -> I32Query<'a,C,
i32, 12>
{
    I32Query
    {
        client, params: [name,nickname,father,mother,breed,gender,dob,alive,thumbnail,images,health,titles,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::ArraySql<Item = T6>,T8: cornucopia_async::StringSql,T9: cornucopia_async::ArraySql<Item = T8>,T10: cornucopia_async::StringSql,T11: cornucopia_async::ArraySql<Item = T10>,> cornucopia_async::Params<'a,
InsertDogParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>, I32Query<'a, C,
i32, 12>, C> for InsertDogStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertDogParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>) -> I32Query<'a, C,
    i32, 12>
    { self.bind(client, &params.name,&params.nickname,&params.father,&params.mother,&params.breed,&params.gender,&params.dob,&params.alive,&params.thumbnail,&params.images,&params.health,&params.titles,) }
}pub fn update_dog() -> UpdateDogStmt
{ UpdateDogStmt(cornucopia_async::private::Stmt::new("UPDATE
	dogs
SET
	name = $1,
	nickname = $2,
	father = $3,
	mother = $4,
	breed = $5,
	gender = $6,
	dob = $7,
	alive = $8,
	thumbnail = $9,
	images = $10,
	health = $11,
	titles = $12
WHERE
	id = $13")) } pub struct
UpdateDogStmt(cornucopia_async::private::Stmt); impl UpdateDogStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::ArraySql<Item = T6>,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::ArraySql<Item = T8>,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::ArraySql<Item = T10>,>(&'a mut self, client: &'a  C,
name: &'a T1,nickname: &'a T2,father: &'a T3,mother: &'a T4,breed: &'a super::super::types::public::Breed,gender: &'a super::super::types::public::Gender,dob: &'a i64,alive: &'a bool,thumbnail: &'a T5,images: &'a T7,health: &'a T9,titles: &'a T11,id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[name,nickname,father,mother,breed,gender,dob,alive,thumbnail,images,health,titles,id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::ArraySql<Item = T6>,T8: cornucopia_async::StringSql,T9: cornucopia_async::ArraySql<Item = T8>,T10: cornucopia_async::StringSql,T11: cornucopia_async::ArraySql<Item = T10>,>
cornucopia_async::Params<'a, UpdateDogParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpdateDogStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateDogParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.name,&params.nickname,&params.father,&params.mother,&params.breed,&params.gender,&params.dob,&params.alive,&params.thumbnail,&params.images,&params.health,&params.titles,&params.id,)) }
}pub fn delete_dog() -> DeleteDogStmt
{ DeleteDogStmt(cornucopia_async::private::Stmt::new("DELETE FROM
	dogs
WHERE
	id = $1")) } pub struct
DeleteDogStmt(cornucopia_async::private::Stmt); impl DeleteDogStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,]).await
} }}pub mod litter
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct LittersByBreedParams<> { pub id: i32,pub breed: super::super::types::public::Breed,}#[derive( Debug)] pub struct InsertLitterParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> { pub name: T1,pub breed: super::super::types::public::Breed,pub parents_image: T2,pub images: T4,}#[derive( Debug)] pub struct UpdateLitterParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> { pub name: T1,pub breed: super::super::types::public::Breed,pub parents_image: T2,pub images: T4,pub id: i32,}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct AllNames
{ pub id : i32,pub name : String,}pub struct AllNamesBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<AllNamesBorrowed<'a>> for AllNames
{
    fn from(AllNamesBorrowed { id,name,}: AllNamesBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct AllNamesQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> AllNamesBorrowed,
    mapper: fn(AllNamesBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> AllNamesQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(AllNamesBorrowed) -> R) ->
    AllNamesQuery<'a,C,R,N>
    {
        AllNamesQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Litter
{ pub id : i32,pub name : String,pub breed : super::super::types::public::Breed,pub parents_image : String,pub images : Vec<String>,}pub struct LitterBorrowed<'a> { pub id : i32,pub name : &'a str,pub breed : super::super::types::public::Breed,pub parents_image : &'a str,pub images : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<LitterBorrowed<'a>> for Litter
{
    fn from(LitterBorrowed { id,name,breed,parents_image,images,}: LitterBorrowed<'a>) ->
    Self { Self { id,name: name.into(),breed,parents_image: parents_image.into(),images: images.map(|v| v.into()).collect(),} }
}pub struct LitterQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> LitterBorrowed,
    mapper: fn(LitterBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> LitterQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(LitterBorrowed) -> R) ->
    LitterQuery<'a,C,R,N>
    {
        LitterQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn all_names() -> AllNamesStmt
{ AllNamesStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name
FROM
	litters")) } pub struct
AllNamesStmt(cornucopia_async::private::Stmt); impl AllNamesStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> AllNamesQuery<'a,C,
AllNames, 0>
{
    AllNamesQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { AllNamesBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <AllNames>::from(it) },
    }
} }pub fn litter_by_id() -> LitterByIdStmt
{ LitterByIdStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name,
	breed,
	parents_image,
	images
FROM
	litters
WHERE
	id = $1")) } pub struct
LitterByIdStmt(cornucopia_async::private::Stmt); impl LitterByIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> LitterQuery<'a,C,
Litter, 1>
{
    LitterQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { LitterBorrowed { id: row.get(0),name: row.get(1),breed: row.get(2),parents_image: row.get(3),images: row.get(4),} }, mapper: |it| { <Litter>::from(it) },
    }
} }pub fn litters_by_breed() -> LittersByBreedStmt
{ LittersByBreedStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name,
	breed,
	parents_image,
	images
FROM
	litters
WHERE
	id = $1 and breed = $2")) } pub struct
LittersByBreedStmt(cornucopia_async::private::Stmt); impl LittersByBreedStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,breed: &'a super::super::types::public::Breed,) -> LitterQuery<'a,C,
Litter, 2>
{
    LitterQuery
    {
        client, params: [id,breed,], stmt: &mut self.0, extractor:
        |row| { LitterBorrowed { id: row.get(0),name: row.get(1),breed: row.get(2),parents_image: row.get(3),images: row.get(4),} }, mapper: |it| { <Litter>::from(it) },
    }
} }impl <'a, C: GenericClient,> cornucopia_async::Params<'a,
LittersByBreedParams<>, LitterQuery<'a, C,
Litter, 2>, C> for LittersByBreedStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    LittersByBreedParams<>) -> LitterQuery<'a, C,
    Litter, 2>
    { self.bind(client, &params.id,&params.breed,) }
}pub fn insert_litter() -> InsertLitterStmt
{ InsertLitterStmt(cornucopia_async::private::Stmt::new("INSERT INTO
	litters(
		name,
		breed,
		parents_image,
		images
	)
VALUES(
	$1,
	$2,
	$3,
	$4)
RETURNING
	id")) } pub struct
InsertLitterStmt(cornucopia_async::private::Stmt); impl InsertLitterStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::ArraySql<Item = T3>,>(&'a mut self, client: &'a  C,
name: &'a T1,breed: &'a super::super::types::public::Breed,parents_image: &'a T2,images: &'a T4,) -> I32Query<'a,C,
i32, 4>
{
    I32Query
    {
        client, params: [name,breed,parents_image,images,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> cornucopia_async::Params<'a,
InsertLitterParams<T1,T2,T3,T4,>, I32Query<'a, C,
i32, 4>, C> for InsertLitterStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertLitterParams<T1,T2,T3,T4,>) -> I32Query<'a, C,
    i32, 4>
    { self.bind(client, &params.name,&params.breed,&params.parents_image,&params.images,) }
}pub fn update_litter() -> UpdateLitterStmt
{ UpdateLitterStmt(cornucopia_async::private::Stmt::new("UPDATE
	litters
SET
	name = $1,
	breed = $2,
	parents_image = $3,
	images = $4
WHERE
	id = $5")) } pub struct
UpdateLitterStmt(cornucopia_async::private::Stmt); impl UpdateLitterStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::ArraySql<Item = T3>,>(&'a mut self, client: &'a  C,
name: &'a T1,breed: &'a super::super::types::public::Breed,parents_image: &'a T2,images: &'a T4,id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[name,breed,parents_image,images,id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,>
cornucopia_async::Params<'a, UpdateLitterParams<T1,T2,T3,T4,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpdateLitterStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateLitterParams<T1,T2,T3,T4,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.name,&params.breed,&params.parents_image,&params.images,&params.id,)) }
}pub fn delete_litter() -> DeleteLitterStmt
{ DeleteLitterStmt(cornucopia_async::private::Stmt::new("DELETE FROM
	litters
WHERE
	id = $1")) } pub struct
DeleteLitterStmt(cornucopia_async::private::Stmt); impl DeleteLitterStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,]).await
} }}pub mod news
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct NNewsOlderThanParams<> { pub date: i64,pub n: i64,}#[derive( Debug)] pub struct InsertNewsParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> { pub title: T1,pub text: T2,pub date: i64,pub images: T4,}#[derive( Debug)] pub struct UpdateNewsParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> { pub title: T1,pub text: T2,pub date: i64,pub images: T4,pub id: i32,}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct AllTitles
{ pub id : i32,pub title : String,}pub struct AllTitlesBorrowed<'a> { pub id : i32,pub title : &'a str,}
impl<'a> From<AllTitlesBorrowed<'a>> for AllTitles
{
    fn from(AllTitlesBorrowed { id,title,}: AllTitlesBorrowed<'a>) ->
    Self { Self { id,title: title.into(),} }
}pub struct AllTitlesQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> AllTitlesBorrowed,
    mapper: fn(AllTitlesBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> AllTitlesQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(AllTitlesBorrowed) -> R) ->
    AllTitlesQuery<'a,C,R,N>
    {
        AllTitlesQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Article
{ pub id : i32,pub title : String,pub text : String,pub date : i64,pub images : Vec<String>,}pub struct ArticleBorrowed<'a> { pub id : i32,pub title : &'a str,pub text : &'a str,pub date : i64,pub images : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<ArticleBorrowed<'a>> for Article
{
    fn from(ArticleBorrowed { id,title,text,date,images,}: ArticleBorrowed<'a>) ->
    Self { Self { id,title: title.into(),text: text.into(),date,images: images.map(|v| v.into()).collect(),} }
}pub struct ArticleQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> ArticleBorrowed,
    mapper: fn(ArticleBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> ArticleQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(ArticleBorrowed) -> R) ->
    ArticleQuery<'a,C,R,N>
    {
        ArticleQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn all_titles() -> AllTitlesStmt
{ AllTitlesStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	title
FROM
	news")) } pub struct
AllTitlesStmt(cornucopia_async::private::Stmt); impl AllTitlesStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> AllTitlesQuery<'a,C,
AllTitles, 0>
{
    AllTitlesQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { AllTitlesBorrowed { id: row.get(0),title: row.get(1),} }, mapper: |it| { <AllTitles>::from(it) },
    }
} }pub fn n_news_older_than() -> NNewsOlderThanStmt
{ NNewsOlderThanStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	title,
	text,
	date,
	images
FROM
	news
WHERE
	date < $1
LIMIT
	$2")) } pub struct
NNewsOlderThanStmt(cornucopia_async::private::Stmt); impl NNewsOlderThanStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
date: &'a i64,n: &'a i64,) -> ArticleQuery<'a,C,
Article, 2>
{
    ArticleQuery
    {
        client, params: [date,n,], stmt: &mut self.0, extractor:
        |row| { ArticleBorrowed { id: row.get(0),title: row.get(1),text: row.get(2),date: row.get(3),images: row.get(4),} }, mapper: |it| { <Article>::from(it) },
    }
} }impl <'a, C: GenericClient,> cornucopia_async::Params<'a,
NNewsOlderThanParams<>, ArticleQuery<'a, C,
Article, 2>, C> for NNewsOlderThanStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    NNewsOlderThanParams<>) -> ArticleQuery<'a, C,
    Article, 2>
    { self.bind(client, &params.date,&params.n,) }
}pub fn insert_news() -> InsertNewsStmt
{ InsertNewsStmt(cornucopia_async::private::Stmt::new("INSERT INTO
	news(
		title,
		text,
		date,
		images
	)
VALUES(
	$1,
	$2,
	$3,
	$4)
RETURNING
	id")) } pub struct
InsertNewsStmt(cornucopia_async::private::Stmt); impl InsertNewsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::ArraySql<Item = T3>,>(&'a mut self, client: &'a  C,
title: &'a T1,text: &'a T2,date: &'a i64,images: &'a T4,) -> I32Query<'a,C,
i32, 4>
{
    I32Query
    {
        client, params: [title,text,date,images,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> cornucopia_async::Params<'a,
InsertNewsParams<T1,T2,T3,T4,>, I32Query<'a, C,
i32, 4>, C> for InsertNewsStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertNewsParams<T1,T2,T3,T4,>) -> I32Query<'a, C,
    i32, 4>
    { self.bind(client, &params.title,&params.text,&params.date,&params.images,) }
}pub fn update_news() -> UpdateNewsStmt
{ UpdateNewsStmt(cornucopia_async::private::Stmt::new("UPDATE
	news
SET
	title = $1,
	text = $2,
	date = $3,
	images = $4
WHERE
	id = $5")) } pub struct
UpdateNewsStmt(cornucopia_async::private::Stmt); impl UpdateNewsStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::ArraySql<Item = T3>,>(&'a mut self, client: &'a  C,
title: &'a T1,text: &'a T2,date: &'a i64,images: &'a T4,id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[title,text,date,images,id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,>
cornucopia_async::Params<'a, UpdateNewsParams<T1,T2,T3,T4,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpdateNewsStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateNewsParams<T1,T2,T3,T4,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.title,&params.text,&params.date,&params.images,&params.id,)) }
}pub fn delete_news() -> DeleteNewsStmt
{ DeleteNewsStmt(cornucopia_async::private::Stmt::new("DELETE FROM
	news
WHERE
	id = $1")) } pub struct
DeleteNewsStmt(cornucopia_async::private::Stmt); impl DeleteNewsStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,]).await
} }}pub mod puppy
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertPuppyParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub litter_id: i32,pub name: T1,pub gender: super::super::types::public::Gender,pub availability: super::super::types::public::Availability,pub image: T2,}#[derive( Debug)] pub struct UpdatePuppyParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub litter_id: i32,pub name: T1,pub gender: super::super::types::public::Gender,pub availability: super::super::types::public::Availability,pub thumbnail: T2,pub id: i32,}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct NamesByLitter
{ pub id : i32,pub name : String,}pub struct NamesByLitterBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<NamesByLitterBorrowed<'a>> for NamesByLitter
{
    fn from(NamesByLitterBorrowed { id,name,}: NamesByLitterBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct NamesByLitterQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> NamesByLitterBorrowed,
    mapper: fn(NamesByLitterBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> NamesByLitterQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(NamesByLitterBorrowed) -> R) ->
    NamesByLitterQuery<'a,C,R,N>
    {
        NamesByLitterQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Litter
{ pub id : i32,pub litter_id : i32,pub name : String,pub gender : super::super::types::public::Gender,pub availability : super::super::types::public::Availability,pub image : String,}pub struct LitterBorrowed<'a> { pub id : i32,pub litter_id : i32,pub name : &'a str,pub gender : super::super::types::public::Gender,pub availability : super::super::types::public::Availability,pub image : &'a str,}
impl<'a> From<LitterBorrowed<'a>> for Litter
{
    fn from(LitterBorrowed { id,litter_id,name,gender,availability,image,}: LitterBorrowed<'a>) ->
    Self { Self { id,litter_id,name: name.into(),gender,availability,image: image.into(),} }
}pub struct LitterQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> LitterBorrowed,
    mapper: fn(LitterBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> LitterQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(LitterBorrowed) -> R) ->
    LitterQuery<'a,C,R,N>
    {
        LitterQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn names_by_litter() -> NamesByLitterStmt
{ NamesByLitterStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	name
FROM
	puppies
WHERE
	litter_id = $1")) } pub struct
NamesByLitterStmt(cornucopia_async::private::Stmt); impl NamesByLitterStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
litter_id: &'a i32,) -> NamesByLitterQuery<'a,C,
NamesByLitter, 1>
{
    NamesByLitterQuery
    {
        client, params: [litter_id,], stmt: &mut self.0, extractor:
        |row| { NamesByLitterBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <NamesByLitter>::from(it) },
    }
} }pub fn puppies_by_litter() -> PuppiesByLitterStmt
{ PuppiesByLitterStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	image
FROM
	puppies
WHERE
	litter_id = $1")) } pub struct
PuppiesByLitterStmt(cornucopia_async::private::Stmt); impl PuppiesByLitterStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
litter_id: &'a i32,) -> LitterQuery<'a,C,
Litter, 1>
{
    LitterQuery
    {
        client, params: [litter_id,], stmt: &mut self.0, extractor:
        |row| { LitterBorrowed { id: row.get(0),litter_id: row.get(1),name: row.get(2),gender: row.get(3),availability: row.get(4),image: row.get(5),} }, mapper: |it| { <Litter>::from(it) },
    }
} }pub fn avaliable_puppies_by_litter() -> AvaliablePuppiesByLitterStmt
{ AvaliablePuppiesByLitterStmt(cornucopia_async::private::Stmt::new("SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	image
FROM
	puppies
WHERE
	litter_id = $1 and availability = 'Available'")) } pub struct
AvaliablePuppiesByLitterStmt(cornucopia_async::private::Stmt); impl AvaliablePuppiesByLitterStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
litter_id: &'a i32,) -> LitterQuery<'a,C,
Litter, 1>
{
    LitterQuery
    {
        client, params: [litter_id,], stmt: &mut self.0, extractor:
        |row| { LitterBorrowed { id: row.get(0),litter_id: row.get(1),name: row.get(2),gender: row.get(3),availability: row.get(4),image: row.get(5),} }, mapper: |it| { <Litter>::from(it) },
    }
} }pub fn insert_puppy() -> InsertPuppyStmt
{ InsertPuppyStmt(cornucopia_async::private::Stmt::new("INSERT INTO
	puppies(
		litter_id,
		name,
		gender,
		availability,
		image
	)
VALUES(
	$1,
	$2,
	$3,
	$4,
	$5)
RETURNING
	id")) } pub struct
InsertPuppyStmt(cornucopia_async::private::Stmt); impl InsertPuppyStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
litter_id: &'a i32,name: &'a T1,gender: &'a super::super::types::public::Gender,availability: &'a super::super::types::public::Availability,image: &'a T2,) -> I32Query<'a,C,
i32, 5>
{
    I32Query
    {
        client, params: [litter_id,name,gender,availability,image,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
InsertPuppyParams<T1,T2,>, I32Query<'a, C,
i32, 5>, C> for InsertPuppyStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertPuppyParams<T1,T2,>) -> I32Query<'a, C,
    i32, 5>
    { self.bind(client, &params.litter_id,&params.name,&params.gender,&params.availability,&params.image,) }
}pub fn update_puppy() -> UpdatePuppyStmt
{ UpdatePuppyStmt(cornucopia_async::private::Stmt::new("UPDATE
	puppies
SET
	litter_id = $1,
	name = $2,
	gender = $3,
	availability = $4,
	image = $5
WHERE
	id = $6")) } pub struct
UpdatePuppyStmt(cornucopia_async::private::Stmt); impl UpdatePuppyStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
litter_id: &'a i32,name: &'a T1,gender: &'a super::super::types::public::Gender,availability: &'a super::super::types::public::Availability,thumbnail: &'a T2,id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[litter_id,name,gender,availability,thumbnail,id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpdatePuppyParams<T1,T2,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpdatePuppyStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdatePuppyParams<T1,T2,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.litter_id,&params.name,&params.gender,&params.availability,&params.thumbnail,&params.id,)) }
}pub fn delete_puppy() -> DeletePuppyStmt
{ DeletePuppyStmt(cornucopia_async::private::Stmt::new("DELETE FROM
	puppies
WHERE
	id = $1")) } pub struct
DeletePuppyStmt(cornucopia_async::private::Stmt); impl DeletePuppyStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,]).await
} }}}