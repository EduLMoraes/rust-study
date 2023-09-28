-- MySQL dump 10.13  Distrib 8.0.34, for Linux (x86_64)
--
-- Host: 127.0.0.1    Database: study
-- ------------------------------------------------------
-- Server version	8.0.34

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `author`
--

DROP TABLE IF EXISTS `author`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `author` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=83 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `author`
--

LOCK TABLES `author` WRITE;
/*!40000 ALTER TABLE `author` DISABLE KEYS */;
INSERT INTO `author` VALUES (1,'Augusto Cury'),(3,'Chuck Wendig'),(72,'Paulo Coelho'),(73,'Ana Cristina Vargas'),(74,'Arthur Conan Doyle'),(75,'Augusto'),(79,'teste'),(80,'tes');
/*!40000 ALTER TABLE `author` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `books`
--

DROP TABLE IF EXISTS `books`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `books` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `id_publishing_company` bigint NOT NULL,
  `id_author` bigint NOT NULL,
  `title` varchar(255) NOT NULL,
  `category` varchar(255) NOT NULL,
  `edition` varchar(100) NOT NULL,
  `publish_date` varchar(255) NOT NULL,
  `description` varchar(255) NOT NULL,
  `qnt_specimens` int NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `books`
--

LOCK TABLES `books` WRITE;
/*!40000 ALTER TABLE `books` DISABLE KEYS */;
INSERT INTO `books` VALUES (1,1,1,'As Armadilhas da Mente','Romance','1','2013','Camile é uma mulher bvela, rica e brilhante, capaz de deixar as pessoas impressionadas com sua habilidade de debate e argumentar. \nMas seus diplomas e seu intelecto não foram suficientes para evitar que se tornasse vítima de suas próprias emoçoes.',1),(2,3,3,'Star Wars: Marcas da Guerra','Ficção Científica','1','2015','Às vésperas de STAR WARS: O despertar da Força, os fãs têm a oportunidade de\n                        acompanhar histórias inéditas ambientadas no universo criado por GeorgeLucas.',1),(40,63,1,'O Vendedor de Sonhos: O Chamado','Ficção Brasileira','5','2021',' O romance mais vendido de Augusto Cury, que deu origem ao filme de Jayme Monjardim, com Dan Stulbach e César Troncoso nos papéis principais. Edição revista pelo autor, com passagens inéditas presentes na adaptação para o cinema.',1),(41,64,72,'Ser como o Rio que Flui...','Ficção Brasileira','1','2007',' Coleção Paulo Coelho',1),(42,64,72,'A Bruxa de Portobello','Ficção Brasileira','1','2007',' Coleção - Antes que todos estes depoimentos saíssem da minha mesa de trabalho e seguissem o destino que eu havia determinado para eles, pensei em transformá-los em um livro tradicional, onde uma estória real é contada depois de exaustiva pesquisa.',1),(43,64,72,'Brida','Ficção Brasileira','1','2007',' Coleção Paulo Coelho',1),(44,65,73,'O Bispo','Religião e Espiritualidade','1','2014',' Até que ponto você aceitaria explorar sua humanidade? Quanto do que pensa, sente ou vive aceitaria tornar público? Ricardo aceitou o desafio e revela-se aos leitores contando segredos que levou para o túmulo.',1),(45,66,74,'Sherlock Holmes: O Vale do Medo','Drama','1','2018',' \"Este é o Vale do Medo, o vale da morte. O terror está nos corações das pessoas do crepúsculo ao redor do dia.\" É nesse tom sombrio que se desenrola a surpreendente narrativa deste último caso investigado por Sherlock Holmes.',1);
/*!40000 ALTER TABLE `books` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `permissions`
--

DROP TABLE IF EXISTS `permissions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `permissions` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `level` varchar(50) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `permissions`
--

LOCK TABLES `permissions` WRITE;
/*!40000 ALTER TABLE `permissions` DISABLE KEYS */;
INSERT INTO `permissions` VALUES (1,'admin'),(2,'librarian'),(3,'user');
/*!40000 ALTER TABLE `permissions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `publishing_company`
--

DROP TABLE IF EXISTS `publishing_company`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `publishing_company` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `name` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=75 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `publishing_company`
--

LOCK TABLES `publishing_company` WRITE;
/*!40000 ALTER TABLE `publishing_company` DISABLE KEYS */;
INSERT INTO `publishing_company` VALUES (1,'Arqueiro'),(3,'Aleph'),(63,'Planeta'),(64,'gold'),(65,'Vida & Concsciência'),(66,'Lafonte'),(67,'Arque'),(71,'teste'),(72,'tes');
/*!40000 ALTER TABLE `publishing_company` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `id_permission` int NOT NULL,
  `id_ebook` bigint DEFAULT '0',
  `name` varchar(50) DEFAULT NULL,
  `surname` varchar(255) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=113 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (7,1,0,'Eduardo','Moraes','0129','17eduardo05@gmail.com'),(107,2,0,'Priscila','Martins','123','priscila@hotmail.com'),(110,3,0,'Bryan','Moraes','bryan123','bryan@gmail.com'),(111,3,0,'Priscila','Bonilha','b09','bonilha@gmail.com'),(112,3,0,'Eduardo','Moraes','123','lmoraeseduardo@gmail.com');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-09-28 16:45:45
