from airflow import DAG
from datetime import datetime, timedelta

from airflow import models
from airflow.kubernetes.secret import Secret
from airflow.providers.cncf.kubernetes.operators.kubernetes_pod import (
    KubernetesPodOperator,
)
from airflow.operators.dummy import DummyOperator

from kubernetes.client import models as k8s
from airflow.utils.dates import days_ago


class KOP(KubernetesPodOperator):
    template_ext = ()


default_args = {
    "owner": "Mohammad Hosein",
    "depends_on_past": False,
    "start_date": "2022-01-19",
    "email": ["mo.zarei@digikala.com"],
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 0,
    "retry_delay": timedelta(minutes=5),
}


dag = DAG(
    "buying_habits_<CICD_HASH_PLACEHOLDER>",  # do not change!
    default_args=default_args,
    schedule_interval="00 15 * * *",
    max_active_runs=1,
    concurrency=10,
    catchup=False,
)

start = DummyOperator(task_id="start", dag=dag)


volume_mounts = ["<CICD_DAG_VOLUME_MOUNTS_PLACEHOLDER>"]  # do not change!

volumes = ["<CICD_DAG_VOLUMES_PLACEHOLDER>"]  # do not change!

environments = {
    # your public environment variables here (dict)
}


cmd_0 = "python src/user_product_category.py".split()
user_product_category = KOP(
    namespace="air",
    image="<CICD_IMAGE_PLACEHOLDER>",  # do not change!
    cmds=cmd_0[0:1],
    arguments=cmd_0[1:],
    labels={"team": "ds"},
    volumes=volumes,
    volume_mounts=volume_mounts,
    env_vars=environments,
    name="user_product_category",
    task_id="user_product_category",
    dag=dag,
    get_logs=True,
    in_cluster=True,
    is_delete_operator_pod=True,
)

cmd_1 = "python src/product_category.py".split()
product_category = KOP(
    namespace="air",
    image="<CICD_IMAGE_PLACEHOLDER>",  # do not change!
    cmds=cmd_1[0:1],
    arguments=cmd_1[1:],
    labels={"team": "ds"},
    volumes=volumes,
    volume_mounts=volume_mounts,
    env_vars=environments,
    name="product_category",
    task_id="product_category",
    dag=dag,
    get_logs=True,
    in_cluster=True,
    is_delete_operator_pod=True,
)

cmd_2 = "python src/users_with_one_purchase.py".split()
users_with_one_purchase = KOP(
    namespace="air",
    image="<CICD_IMAGE_PLACEHOLDER>",  # do not change!
    cmds=cmd_2[0:1],
    arguments=cmd_2[1:],
    labels={"team": "ds"},
    volumes=volumes,
    volume_mounts=volume_mounts,
    env_vars=environments,
    name="users_with_one_purchase",
    task_id="users_with_one_purchase",
    dag=dag,
    get_logs=True,
    in_cluster=True,
    is_delete_operator_pod=True,
)

cmd_3 = "python src/save_suggestions_to_redis.py".split()
save_suggestions_to_redis = KOP(
    namespace="air",
    image="<CICD_IMAGE_PLACEHOLDER>",  # do not change!
    cmds=cmd_3[0:1],
    arguments=cmd_3[1:],
    labels={"team": "ds"},
    volumes=volumes,
    volume_mounts=volume_mounts,
    env_vars=environments,
    name="save_suggestions_to_redis",
    task_id="save_suggestions_to_redis",
    dag=dag,
    get_logs=True,
    in_cluster=True,
    is_delete_operator_pod=True,
)


(
    start
    >> user_product_category
    >> product_category
    >> users_with_one_purchase
    >> save_suggestions_to_redis
)
