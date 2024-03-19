--
-- PostgreSQL database dump
--

-- Dumped from database version 16.2 (Debian 16.2-1.pgdg110+2)
-- Dumped by pg_dump version 16.2 (Debian 16.2-1.pgdg110+2)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: tiger; Type: SCHEMA; Schema: -; Owner: admin
--

CREATE SCHEMA tiger;


ALTER SCHEMA tiger OWNER TO admin;

--
-- Name: tiger_data; Type: SCHEMA; Schema: -; Owner: admin
--

CREATE SCHEMA tiger_data;


ALTER SCHEMA tiger_data OWNER TO admin;

--
-- Name: topology; Type: SCHEMA; Schema: -; Owner: admin
--

CREATE SCHEMA topology;


ALTER SCHEMA topology OWNER TO admin;

--
-- Name: SCHEMA topology; Type: COMMENT; Schema: -; Owner: admin
--

COMMENT ON SCHEMA topology IS 'PostGIS Topology schema';


--
-- Name: fuzzystrmatch; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS fuzzystrmatch WITH SCHEMA public;


--
-- Name: EXTENSION fuzzystrmatch; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION fuzzystrmatch IS 'determine similarities and distance between strings';


--
-- Name: postgis; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA public;


--
-- Name: EXTENSION postgis; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION postgis IS 'PostGIS geometry and geography spatial types and functions';


--
-- Name: postgis_tiger_geocoder; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS postgis_tiger_geocoder WITH SCHEMA tiger;


--
-- Name: EXTENSION postgis_tiger_geocoder; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION postgis_tiger_geocoder IS 'PostGIS tiger geocoder and reverse geocoder';


--
-- Name: postgis_topology; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS postgis_topology WITH SCHEMA topology;


--
-- Name: EXTENSION postgis_topology; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION postgis_topology IS 'PostGIS topology spatial types and functions';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: items; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.items (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    list_id uuid,
    name character varying(255) NOT NULL,
    done boolean DEFAULT false NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    lat double precision,
    lng double precision,
    geom public.geometry(Point,2950),
    category smallint
);


ALTER TABLE public.items OWNER TO admin;

--
-- Name: lists; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.lists (
    id uuid NOT NULL,
    title character varying(255) NOT NULL,
    user_id uuid NOT NULL
);


ALTER TABLE public.lists OWNER TO admin;

--
-- Name: users; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255) NOT NULL,
    username character varying NOT NULL,
    password character varying NOT NULL,
    roles character varying NOT NULL
);


ALTER TABLE public.users OWNER TO admin;

--
-- Data for Name: items; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.items (id, list_id, name, done, deleted, lat, lng, geom, category) FROM stdin;
e2ba29ea-d2f1-4944-b2ad-a5690525ecea	b17f884b-dd39-40f3-907e-bc7b274c1f40	Milk	f	f	\N	\N	\N	\N
7d30141d-120e-44a0-a43b-356847d46adc	509eec5e-df3d-4f27-b0aa-54df81aa2381	Steak	t	f	55.3530621	10.3442499	0101000020860B0000827A8E2331AD4B40734DDC8541B02440	1
e1b34b48-7a74-481e-a8e8-159c07916b93	509eec5e-df3d-4f27-b0aa-54df81aa2381	Chicken	t	f	55.3530621	10.3442499	0101000020860B0000827A8E2331AD4B40734DDC8541B02440	1
d0117d00-d160-4da7-b9cc-582179cf8167	509eec5e-df3d-4f27-b0aa-54df81aa2381	Pasta	t	f	55.3530628	10.3442416	0101000020860B000093B76D2931AD4B406ED85B6F40B02440	2
c6cd99d8-800f-4c06-a1ab-f10d92279e3f	509eec5e-df3d-4f27-b0aa-54df81aa2381	Rice	t	f	55.3530628	10.3442416	0101000020860B000093B76D2931AD4B406ED85B6F40B02440	2
f3afd508-bfad-4f51-b8d1-ec600820c0a7	509eec5e-df3d-4f27-b0aa-54df81aa2381	Cheese	t	f	55.3530717	10.3442589	0101000020860B00005252167431AD4B40C0B6D9B342B02440	3
7d565bc9-ce61-4947-8846-bcd79b9736b5	509eec5e-df3d-4f27-b0aa-54df81aa2381	Milk	t	f	55.3530717	10.3442589	0101000020860B00005252167431AD4B40C0B6D9B342B02440	3
3bc37035-ee30-4764-89d4-7fe1ee539589	509eec5e-df3d-4f27-b0aa-54df81aa2381	Cola	t	f	55.3531045	10.3443968	0101000020860B000004C93B8732AD4B40B1B101C754B02440	4
\.


--
-- Data for Name: lists; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.lists (id, title, user_id) FROM stdin;
509eec5e-df3d-4f27-b0aa-54df81aa2381	Shopping	c54a5ed4-d788-4a51-a18b-778f0b2d61bc
509eec5e-df3d-4f27-b0aa-54df81aa2381	Shopping	2e86e5e4-530e-40c1-905d-6c45bf38feda
b17f884b-dd39-40f3-907e-bc7b274c1f40	Personal	c54a5ed4-d788-4a51-a18b-778f0b2d61bc
\.


--
-- Data for Name: spatial_ref_sys; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.spatial_ref_sys (srid, auth_name, auth_srid, srtext, proj4text) FROM stdin;
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.users (id, first_name, last_name, username, password, roles) FROM stdin;
c54a5ed4-d788-4a51-a18b-778f0b2d61bc	Anders	Madsen	anderslm@hotmail.com	password	admin,user
2e86e5e4-530e-40c1-905d-6c45bf38feda	Nadia	Thomsen	nadiasophie@hotmail.com	password	admin,user
\.


--
-- Data for Name: geocode_settings; Type: TABLE DATA; Schema: tiger; Owner: admin
--

COPY tiger.geocode_settings (name, setting, unit, category, short_desc) FROM stdin;
\.


--
-- Data for Name: pagc_gaz; Type: TABLE DATA; Schema: tiger; Owner: admin
--

COPY tiger.pagc_gaz (id, seq, word, stdword, token, is_custom) FROM stdin;
\.


--
-- Data for Name: pagc_lex; Type: TABLE DATA; Schema: tiger; Owner: admin
--

COPY tiger.pagc_lex (id, seq, word, stdword, token, is_custom) FROM stdin;
\.


--
-- Data for Name: pagc_rules; Type: TABLE DATA; Schema: tiger; Owner: admin
--

COPY tiger.pagc_rules (id, rule, is_custom) FROM stdin;
\.


--
-- Data for Name: topology; Type: TABLE DATA; Schema: topology; Owner: admin
--

COPY topology.topology (id, name, srid, "precision", hasz) FROM stdin;
\.


--
-- Data for Name: layer; Type: TABLE DATA; Schema: topology; Owner: admin
--

COPY topology.layer (topology_id, layer_id, schema_name, table_name, feature_column, feature_type, level, child_id) FROM stdin;
\.


--
-- Name: topology_id_seq; Type: SEQUENCE SET; Schema: topology; Owner: admin
--

SELECT pg_catalog.setval('topology.topology_id_seq', 1, false);


--
-- Name: items items_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT items_pkey PRIMARY KEY (id);


--
-- Name: lists lists_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.lists
    ADD CONSTRAINT lists_pkey PRIMARY KEY (id, user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: items_geom_idx; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX items_geom_idx ON public.items USING gist (geom);


--
-- Name: items_geom_idx1; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX items_geom_idx1 ON public.items USING gist (geom);


--
-- Name: lists lists_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.lists
    ADD CONSTRAINT lists_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

